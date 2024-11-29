use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::error::Error;

use crate::types::SpotifyApiResponse;

async fn spotify_api_request(
    endpoint: String,
    authorization: String,
) -> Result<SpotifyApiResponse, Box<dyn Error>> {
    let client = Client::new();

    // Build headers
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(&authorization)?);

    // Make the GET request
    let response = client
        .get(endpoint)
        .headers(headers)
        .send()
        .await?;

    // Check for HTTP success
    if response.status().is_success() {
        // Deserialize JSON response
        let json_body: SpotifyApiResponse = response.json().await?;
        Ok(json_body)
    } else {
        Err(format!(
            "Request failed with status: {}",
            response.status()
        )
        .into())
    }
}

/// Query top tracks
pub async fn track_query_builder(authorization: &str) -> Result<SpotifyApiResponse, Box<dyn Error>> {
    let endpoint = String::from("https://api.spotify.com/v1/me/top/tracks");
    let authorization = format!("Bearer {}", authorization);
    let query =   spotify_api_request(endpoint, authorization).await?;
    Ok(query)
}

/// Query top artists
pub async fn artist_query_builder(authorization: &str) -> Result<SpotifyApiResponse, Box<dyn Error>> {
    let endpoint = String::from("https://api.spotify.com/v1/me/top/artists");
    let authorization =  format!("Bearer {}", authorization);
    let query =   spotify_api_request(endpoint, authorization).await?;
    Ok(query)
}
