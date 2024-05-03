# Syntax Spotify Transformer

This is a small, quick and dirty Rust program that takes a list of shows from the [Syntax.fm](https://github.com/syntax-fm/website) repo and updates their frontmatter with the Spotify URLs for each episode.

## Setup

### Authorization

All Spotify endpoints require app authorization; you will need to generate a token that indicates that the client has been granted permission to perform requests. You can start by [registering your app to get the necessary client credentials](https://developer.spotify.com/dashboard/applications).

Set the Redirect URI to `http://localhost:8080/callback` and the select the `Web API` SDK type.

### Environment Variables

The program requires the following environment variables to be set:

- `RSPOTIFY_CLIENT_ID`: The client ID from the Spotify dashboard
- `RSPOTIFY_CLIENT_SECRET`: The client secret from the Spotify dashboard

You must set these variables in your shell.

## Usage

1. Clone the repo
2. Run `cargo install --path .` to build and install the program globally (or run `cargo run` to run the program locally for testing but you'll need to make sure you have a `shows/` folder in the root directory)
3. Go into the **root directory** of the Syntax repository and run `syntax-spotify-transformer` to start the program
4. The first time the program runs, it will prompt you to authorize the app to access your Spotify account. You will be redirected to a URL that looks like `http://localhost:8080/callback?code=...` You must copy this URL and paste it into your terminal.
   ![Example Authorization Flow](https://raw.githubusercontent.com/ramsayleung/rspotify/master/doc/images/rspotify.gif)
5. The program will run until it has updated all the shows

## Notes

- I haven't really implemented error handling, so if the program fails to run for any reason, it will probably panic.
- If the program fails to process a show, or multiple entries are found for a given date, it will print the list of skipped shows to the console at the end. These shows will require manual intervention to update the frontmatter.
