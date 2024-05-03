use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::{Country, Market, ShowId},
    scopes, AuthCodeSpotify, Config, Credentials, OAuth,
};
use syntax_spotify_transformer::utils;
use tokio::time::{sleep, Duration};

const SYNTAX_SHOW_ID: &str = "4kYCRYJ3yK5DQbP5tbfZby";

async fn run(client: &AuthCodeSpotify) {
    let mut shows_map = utils::generate_hashmap_from_shows();
    println!("📝 Loaded {} shows from markdown files", shows_map.len());

    let mut skipped_shows: Vec<String> = vec![];

    let limit = 50;
    let mut offset = 0;

    loop {
        // Gets the first batch of episodes from Spotify
        let episodes = client
            .get_shows_episodes_manual(
                ShowId::from_id(SYNTAX_SHOW_ID).unwrap(),
                Some(Market::Country(Country::UnitedKingdom)),
                Some(limit),
                Some(offset),
            )
            .await
            .unwrap();

        // Iterate through the episodes and update the frontmatter
        for episode in episodes.items {
            if !shows_map.contains_key(&episode.release_date) {
                skipped_shows.push(episode.name);
                continue;
            }

            let (title, target) = shows_map.get(&episode.release_date).unwrap();

            match utils::update_frontmatter(
                target,
                "spotify_url",
                episode.external_urls.get("spotify").unwrap(),
            ) {
                Ok(_) => {
                    println!("📝 Successfully updated \"{}\"", title);
                    shows_map.remove(&episode.release_date);
                }
                Err(e) => println!("🛑 Oops something went wrong: {}", e),
            }
        }

        // Iteration ends when the `next` field is `none`
        if episodes.next.is_none() {
            break;
        }

        offset += limit;

        // Attempt to prevent rate limiting
        println!("🔄 Pausing iteration for 3 seconds");
        sleep(Duration::from_secs(3)).await;
    }

    println!(
        "🛑 Skipped {} shows, please update the following manually:",
        skipped_shows.len()
    );
    for show in skipped_shows {
        println!("🛑 {}", show);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let credentials = match Credentials::from_env() {
        Some(credentials) => {
            println!("🔑 Credentials loaded successfully");
            credentials
        }
        _ => {
            panic!("🛑 Error reading credentials!");
        }
    };

    let oauth = OAuth {
        redirect_uri: "http://localhost:8080/callback".to_string(),
        scopes: scopes!("user-read-playback-position"),
        ..Default::default()
    };

    let config = Config {
        token_cached: true,
        ..Default::default()
    };

    println!("🔑 Starting Spotify auth");

    let client = AuthCodeSpotify::with_config(credentials, oauth, config);
    let url = client.get_authorize_url(false).unwrap();
    client.prompt_for_token(&url).await.unwrap();

    run(&client).await;

    Ok(())
}
