use jsonrpc_core::{ IoHandler, Params, Error};
use serde_json::Value;
use crate::types::TimeRange;

use crate::query_builder::{can_claim_top_tracks, can_claim_top_artist, can_claim_recently_played_track};

pub fn create_io() -> IoHandler {
    let mut io = IoHandler::default();
    
    io.add_method("can_claim_top_tracks", |params: Params| async {
        let (authorization, track_id, time_range, list_range) = 
            params.parse::<(String, String, u8, u8)>()
            .map_err(|e| Error::invalid_params(e.message))?;

        if !(0..=2).contains(&time_range) {
            return Err(Error::invalid_params("Invalid time range"));
        }
        let time_range = TimeRange::from_number(time_range)
        .map_err(|e| Error::invalid_params_with_details(e.to_string(),""))?;

        can_claim_top_tracks(authorization, track_id, time_range,list_range).await
            .map(|result| Value::Bool(result))
            .map_err(|e| Error::invalid_params_with_details(e.to_string(), ""))
    });


    io.add_method("can_claim_top_artist", |params: Params| async {
       let (authorization, artist_id, time_range, list_range) = 
            params.parse::<(String, String, u8, u8)>()
            .map_err(|e| Error::invalid_params(e.message))?;

        if !(0..=2).contains(&time_range) {
            return Err(Error::invalid_params("Invalid time range"));
        }
        let time_range = TimeRange::from_number(time_range)
        .map_err(|e| Error::invalid_params_with_details(e.to_string(),""))?;

        can_claim_top_artist(authorization, artist_id, time_range,list_range).await
            .map(|result| Value::Bool(result))
            .map_err(|e| Error::invalid_params_with_details(e.to_string(), ""))
    });

    io.add_method("can_claim_recently_played_track", |params: Params| async {
        let (authorization, track_id, after, played_time) = 
            params.parse::<(String, String,u64, u8)>()
            .map_err(|e| Error::invalid_params(e.message))?;

        can_claim_recently_played_track(authorization,track_id,after,played_time).await
            .map(|result| Value::Bool(result))
            .map_err(|e| Error::invalid_params_with_details(e.to_string(), ""))
    });

    io
}