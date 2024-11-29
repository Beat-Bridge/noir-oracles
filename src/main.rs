use query_builder::{artist_query_builder, track_query_builder};
use types::SpotifyApiResponse;

pub mod  query_builder;
pub mod types;


#[tokio::main]
async fn main() {
    let cookie = "BQB-NAotVffZiKMy_l85jIE_xqh2rs-GPJIUVEDM5TKK3tZWs8O4KrSipp6s8oCrJ-2wP2r9MNsLpj9LR2QYkE3lhbOAZXdGlCEY32yFwV68lraTdT5TEIi6uGWN6cgKrhUga8oQl2Dbx4U73T8B_M3JZqL1HSty8NEwDLrGvA6WHQfHI__WuLmwvZ3hPBGec08cU-IqwP6-JdCSAuGPbX1oq2Tb-K5IiZZOBSu6lUcvGW7VV0OSFm_zgdhM-lRIUjEKr_nCa3MEvKH-24KUud4kKFK5AK5P3wUTIyoAN9UOnJypyHT_bAmMiMXitDPxwW4WDJ5uCj5tnp4oiC3JcQtOqik";
    let top_tracks:SpotifyApiResponse =  track_query_builder(&cookie).await.unwrap();
    println!("{:?}", top_tracks);

    let top_artists:SpotifyApiResponse  = artist_query_builder(&cookie).await .unwrap();
    println!("{:?}", top_artists);
}