use serde::{Deserialize, Serialize};

pub const  CAN_CLAIM_TOP_TRACKS: &str = "can_claim_top_track";
pub const  CAN_CLAIM_TOP_ARTISTS: &str = "can_claim_top_artist";
pub const  CAN_CLAIM_RECENTLY_PLAYED_TRACK: &str = "can_claim_recently_played_track";

#[derive(Serialize, Deserialize, Debug)]
pub struct RecentlyPlayed {
    href: String,                     // A link to the full result
    limit: u32,                       // Maximum number of items in the response
    next: Option<String>,             // URL to the next page of items
    cursors: Option<Cursors>,         // Cursors for pagination  
    pub items: Vec<PlayedTrack>,  
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cursors {
    after: Option<String>,            // Cursor for the next page
    before: Option<String>,           // Cursor for the previous page
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayedTrack {
    played_at: String,            
    context: Option<TrackContext>,    
    pub track: Track,              
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackContext {
    r#type: String,                   // The type of context (e.g., "artist", "playlist")
    href: Option<String>,             // Link to full details of the context
    external_urls: Option<ExternalUrls>, // External URLs for the context
    uri: Option<String>,              // Spotify URI for the context
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AristsStatsResponse {
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
    pub href: String,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub items: Vec<Artist>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TracksStatsResponse {
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
    pub href: String,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub items: Vec<Track>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalIds {
    isrc: Option<String>,
    ean: Option<String>,
    upc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    pub album_type: String,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_playable: Option<bool>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub total_tracks: u32,
    pub r#type: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub height: u32,
    pub url: String,
    pub width: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    pub album: Album,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_ids: ExternalIds,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub is_local: bool,
    pub is_playable: Option<bool>,
    pub name: String,
    pub popularity: u32,
    pub preview_url: Option<String>,
    pub track_number: u32,
    pub r#type: String,
    pub uri: String,
}

pub enum TimeRange {
    ShortTerm = 0,
    MediumTerm  = 1,
    LongTerm = 2,
}


impl TimeRange {
    /// Converts a numerical value to a TimeRange.
    ///
    /// Returns an error if the given value is not a valid TimeRange.
    pub fn from_number(value: u8) -> Result<Self, String> {
            match value {
                0 => Ok(TimeRange::ShortTerm),
                1 => Ok(TimeRange::MediumTerm),
                2 => Ok(TimeRange::LongTerm),
                _ => Err(format!("Invalid value for TimeRange: {}", value)),
            }
        }
}
