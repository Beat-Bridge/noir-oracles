use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::error::Error;

use crate::types::{RecentlyPlayed, TimeRange, UserStatsResponse, SpotifyApiResponse, ResponseType};

/// Performs a GET request to the Spotify API.
///
/// # Arguments
///
/// * `endpoint` - The URL of the API endpoint to query.
/// * `authorization` - The Bearer token for the user's Spotify API session.
/// * `response_type` - The type of response to expect from the API.
///
/// # Errors
///
/// This function will return an error if the API request fails or if the response
/// is not in the expected format.
async fn spotify_api_request(
    endpoint: String,
    authorization: String,
    response_type: ResponseType,
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
        match response_type {
            ResponseType::UserStats => {
                let json_body: UserStatsResponse = response.json().await?;
                Ok(SpotifyApiResponse::UserStats(json_body))
            }
            ResponseType::RecentlyPlayed => {
                let json_body: RecentlyPlayed = response.json().await?;
                Ok(SpotifyApiResponse::RecentlyPlayed(json_body))
            }
        }
    } else {
        Err(format!(
            "Request failed with status: {}",
            response.status()
        )
        .into())
    }
}


/// Builds a query to the Spotify API to fetch a user's top artists or tracks.
///
/// # Arguments
///
/// * `authorization` - The Bearer token for the user's Spotify API session.
/// * `query_type` - `true` to query for the user's top artists, `false` to query for their top tracks.
/// * `range` - The time range over which to fetch the user's top artists/tracks. (Default: `TimeRange::ShortTerm`)
/// * `limit` - The maximum number of items to return. (Default: 20)
/// * `offset` - The index of the first item to return. (Default: 0)
///
/// # Errors
///
/// This function will return an error if the API request fails or if the response
/// is not in the expected format.
pub async fn stats_query_builder(
    authorization: &str,
    query_type: bool, // true for artists, false for tracks
    range: TimeRange,
    limit: i8,
    offset: i8,
) -> Result<UserStatsResponse, Box<dyn Error>> {
    // Convert `TimeRange` enum to string
    let time_range = match range {
        TimeRange::ShortTerm => "short_term",
        TimeRange::MediumTerm => "medium_term",
        TimeRange::LongTerm => "long_term",
    };

    // Convert `query_type` boolean to corresponding Spotify API type
    let query_type_string = if query_type { "artists" } else { "tracks" };

    // Construct the API endpoint URL
    let endpoint = format!(
        "https://api.spotify.com/v1/me/top/{query_type_string}?time_range={}&limit={}&offset={}",
        time_range, limit, offset
    );

    // Add Bearer token to the authorization header
    let auth_header = format!("Bearer {}", authorization);

    // Perform the API request
    match spotify_api_request(endpoint, auth_header, ResponseType::UserStats).await? {
        SpotifyApiResponse::UserStats(stats) => Ok(stats),
        _ => Err("Unexpected response type".into()),
    }
}



/// Builds a query to the Spotify API to fetch a user's recently played tracks.
///
/// # Arguments
///
/// * `authorization` - The Bearer token for the user's Spotify API session.
/// * `limit` - The maximum number of items to return. (Default: 20)
/// * `before` - The timestamp of the oldest item to return. (Default: None)
/// * `after` - The timestamp of the newest item to return. (Default: None)
///
/// # Errors
///
/// This function will return an error if the API request fails or if the response
/// is not in the expected format.
pub async fn recently_played_query_builder(authorization: &str,  limit: i8, before: i8, after: i64) -> Result<RecentlyPlayed, Box<dyn Error>> {
    let endpoint = format!("https://api.spotify.com/v1/me/player/recently-played?limit={}&after={}&before={}",limit,after,before);
    let auth_header = format!("Bearer {}", authorization);
    match spotify_api_request(endpoint, auth_header, ResponseType::RecentlyPlayed).await? {
        SpotifyApiResponse::RecentlyPlayed(plays) => Ok(plays),
        _ => Err("Unexpected response type".into()),
    }
}