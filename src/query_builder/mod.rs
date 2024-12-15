use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::error::Error;
use serde::de::DeserializeOwned;

use crate::types::{AristsStatsResponse, RecentlyPlayed, TimeRange, TracksStatsResponse};

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

pub async fn spotify_api_request<T>(
    endpoint: String,
    authorization: String,
) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let client = Client::new();

    // Build headers
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(&authorization)?);

    println!("Endpoint: {}, headers: {:#?}", endpoint, headers);
    // Make the GET request
    let response = client
        .get(endpoint)
        .headers(headers)
        .send()
        .await?;

    // Check for HTTP success
    if response.status().is_success() {
        // Deserialize the JSON response
        Ok(response.json::<T>().await?)
    } else {
        // Handle HTTP errors gracefully
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
pub async fn stats_query_builder<T>(
    authorization: String,
    query_type: bool, // true for artists, false for tracks
    range: TimeRange,
    limit: u8,
    offset: u8,
) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
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
    let auth_header = format!("{}", authorization);

    // Perform the API request
   let response = spotify_api_request::<T>(endpoint, auth_header).await?;

    Ok(response)
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
pub async fn recently_played_query_builder(authorization: String, after: u64) -> Result<RecentlyPlayed, Box<dyn Error>> {
    let endpoint = format!("https://api.spotify.com/v1/me/player/recently-played?after={}",after);
    let auth_header = format!("{}", authorization);
    let response = spotify_api_request::<RecentlyPlayed>(endpoint, auth_header).await?;
    Ok(response)
}


/// Checks if the user can claim a given track in the top tracks of a given list range.
///
/// # Arguments
///
/// * `authorization` - The Bearer token for the user's Spotify API session.
/// * `track_id` - The ID of the track to check.
/// * `list_range` - The range of the top tracks list to check. (0 = Last 4 weeks, 1 = Last 6 months, 2 = All time)
///
/// # Errors
///
/// This function will return an error if the API request fails or if the response
/// is not in the expected format.
///
pub async  fn can_claim_top_tracks(authorization: String, track_id: String, time_range:TimeRange, list_range :u8) -> Result<String, Box<dyn Error>> {
    let  query = stats_query_builder::<TracksStatsResponse>(authorization, false, time_range, list_range, 0).await?; 
    for track in query.items {
        if track.id == track_id {
           return Ok(String::from("1"));
        }
    }
     return Ok(String::from("0"));
}



pub async  fn can_claim_top_artist(authorization: String, artist_id: String, time_range:TimeRange, list_range :u8) -> Result<String, Box<dyn Error>> {
    let  query = stats_query_builder::<AristsStatsResponse>(authorization, true, time_range, list_range, 0).await?; 
    for atist in query.items {
        if atist.id == artist_id {
           return Ok(String::from("1"));
        }
    }
    Ok(String::from("0"))
}

pub async  fn can_claim_recently_played_track(authorization: String, track_id: String,  after: u64, played_times: u8) -> Result<String, Box<dyn Error>> {
    let  query = recently_played_query_builder(authorization, after).await?;
    let mut count:u8 = 0; 
    for recently_played in query.items {
        if recently_played.track.id == track_id {
            count += 1;
        }
         
        if count >= played_times {
            return Ok(String::from("1"));
        }
    }
    Ok(String::from("0"))
}