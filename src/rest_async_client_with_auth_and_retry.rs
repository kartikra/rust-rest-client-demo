
use std::time::Duration;

use reqwest::{self, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use reqwest_middleware::{ClientBuilder};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}
#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}

fn print_tracks(tracks: Vec<&Track>) {
    for track in tracks {
        println!("🔥 {}", track.name);
        println!("💿 {}", track.album.name);
        println!(
            "🕺 {}",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("🌎 {}", track.external_urls.spotify);
        println!("---------")
    }
}

// tokio let's us use "async" on our main function
// https://developer.spotify.com/console/get-search-item/?q=Muse&type=track&market=US&limit=5&offset=5&include_external=

#[tokio::main]
pub async fn search_spotify_titles(spotify_auth_token: String){

    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track&market={market}&limit={limit}&offset={offset}",
        // go check out her latest album. It's 🔥
        query = "Little Simz",
        market = "US",
        limit = "50",
        offset = "0"
    );

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let reqwest_client = reqwest::Client::builder()
                                .timeout(Duration::from_secs(60)) // Overall timeout is 60 secs connection + unwrapping json
                                .connect_timeout(Duration::from_secs(40)) // // Specify connection timeout only
                                .build()
                                .expect("Unable to connect");
    
    let client = ClientBuilder::new(reqwest_client)
                                    .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                                    .build();

    // let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, spotify_auth_token)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    
    // Pretty Print Json String
    // println!("Success! {:?}", response);

    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<APIResponse>().await {
                // Ok(parsed) => println!("Success! {:?}", parsed),
                Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token from https://developer.spotify.com/console/get-search-item/");
        }
        other => {
            panic!("Something unexpected happened: {:?}", other);
        }
    }
}

