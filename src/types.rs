use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct  RecentlyPlayed {

}


#[derive(Serialize, Deserialize, Debug)]
pub struct UserStatsResponse {
    total: u32,      
    limit: u32,      
    offset: u32,     
    href: String,   
    next: Option<String>, 
    previous: Option<String>, 
    items: Vec<Item>,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Item {
    Artist(Artist),
    Track(Track),
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalIds {
    isrc: String, 
    ean: String,  
    upc: String,  
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    album_type: String,
    artists: Vec<Artist>,
    available_markets: Vec<String>,
    external_urls: ExternalUrls,
    href: String,
    id: String,
    images: Vec<Image>,
    is_playable: bool,
    name: String,
    release_date: String,
    release_date_precision: String,
    total_tracks: u32,
    r#type: String,
    uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    external_urls: ExternalUrls,
    href: String,
    id: String,
    name: String,
    r#type: String,
    uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    height: u32,
    url: String,
    width: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    album: Album,
    artists: Vec<Artist>,
    available_markets: Vec<String>,
    disc_number: u32,
    duration_ms: u32,
    explicit: bool,
    external_ids: ExternalIds,
    external_urls: ExternalUrls,
    href: String,
    id: String,
    is_local: bool,
    is_playable: bool,
    name: String,
    popularity: u32,
    preview_url: Option<String>,
    track_number: u32,
    r#type: String,
    uri: String,
}


pub enum TimeRange {
    ShortTerm,
    MediumTerm,
    LongTerm,
}
