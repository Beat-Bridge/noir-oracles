use query_builder::{artist_query_builder, track_query_builder};

pub mod  query_builder;
pub mod types;


#[tokio::main]
async fn main() {
    // if let Err(e) = track_query_builder().await {
    //     eprintln!("Failed to query top tracks: {}", e);
    // }

    // if let Err(e) = artist_query_builder().await {
    //     eprintln!("Failed to query top artists: {}", e);
    // }
}