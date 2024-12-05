use core::time;

use jsonrpc_core::{ futures::{future::ok, SinkExt}, Error, IoHandler, Params};
use jsonrpc_http_server::hyper::body::HttpBody;
use serde_json; 
use crate::{redis::store_key_and_token, types::{TimeRange, CAN_CLAIM_RECENTLY_PLAYED_TRACK, CAN_CLAIM_TOP_ARTISTS, CAN_CLAIM_TOP_TRACKS,  }};
use jsonrpc_core::types::Value; 

use crate::query_builder::{can_claim_top_tracks, can_claim_top_artist, can_claim_recently_played_track};

async fn handle_can_claim_top_tracks(params: &serde_json::Value) -> Result<Value, Error> {
    let inputs = params
        .get("inputs")
        .and_then(|v| v.as_array())
        .ok_or_else(|| Error::invalid_params("Missing or invalid 'inputs'"))?;

    if inputs.len() != 4 {
        return Err(Error::invalid_params(
            "Invalid input; requires 4 distinct inputs for 'can_claim_top_tracks'",
        ));
    }

    let auth = inputs[0]
        .as_array()
        .ok_or_else(|| Error::invalid_params("First input must be an array"))?;
    let track = inputs[1]
        .as_array()
        .ok_or_else(|| Error::invalid_params("Second input must be an array"))?;
    let time_range = inputs[2]
        .as_array()
        .ok_or_else(|| Error::invalid_params("Third input must be an array"))?;
    let list_range = inputs[3]
        .as_array()
        .ok_or_else(|| Error::invalid_params("Fourth input must be an array"))?;

    let auth_data: String = auth.iter().map(hex_to_char).collect();
    let track_data: String = track.iter().map(hex_to_char).collect();
    let time_range_data: Vec<u8> = time_range.iter().map(hex_to_u8).collect();
    let list_range_data: Vec<u8> = list_range.iter().map(hex_to_u8).collect();

    if time_range_data.is_empty() || list_range_data.is_empty() {
        return Err(Error::invalid_params("Time range or list range is empty"));
    }

    let time_range_type = TimeRange::from_number(time_range_data[0])
        .map_err(|e| Error::invalid_params_with_details(e.to_string(), ""))?;

    can_claim_top_tracks(auth_data, track_data, time_range_type, list_range_data[0])
        .await
        .map(|result| Value::Bool(result))
        .map_err(|e| Error::invalid_params_with_details(e.to_string(), ""))
}


fn hex_to_u8(hex_string: &Value) -> u8 {
    let hex_str = hex_string.as_str().unwrap_or("\0");
    u8::from_str_radix(&hex_str[2..], 16).unwrap_or(0)
}

fn hex_to_char(hex_string: &Value) -> char {
    let hex_str = match hex_string.as_str() {
        Some(s) => s,
        None => return '\0',
    };

    if !hex_str.starts_with("0x") {
        return '\0';
    }


    let trimmed = &hex_str[2..];
    let number = match u32::from_str_radix(trimmed, 16) {
        Ok(n) => n,
        Err(_) => return '\0', 
    };

    match char::from_u32(number) {
        Some(c) => c,
        None => '\0', 
    }
}



pub fn create_io() -> IoHandler {
    let mut io = IoHandler::default();
    io.add_method("resolve_foreign_call", |params: Params| async {
        match params {
            Params::Array(items) if items.len() == 1 => {
                let params = &items[0];

                if !params.is_object() {
                    return Err(Error::invalid_params("Invalid params; expected an object"));
                }

                let function = params.get("function");
                if let Some(function) = function {
                    if function == CAN_CLAIM_TOP_TRACKS {
                        return handle_can_claim_top_tracks(params).await;
                    } else {
                        return Err(Error::invalid_params("Invalid method"));
                    }
                } else {
                    return Err(Error::invalid_params("Missing 'function' field"));
                }
            }
            _ => Err(Error::invalid_params("Invalid params; expected a single-item array")),
        }
    });

    io.add_method("store_key", |params: Params| async move {
        // Parse the parameters into a tuple of two strings
        let (id, token): (String, String) = 
            params.parse::<(String, String)>()
            .map_err(|e| Error::invalid_params(e.message))?;
        if id.is_empty() || token.is_empty() {
            return Err(Error::invalid_params("ID or token cannot be empty"));
        }
        store_key_and_token(id.clone(), token.clone()).
            map_err(|e| Error::invalid_params(e.to_string()))?;
        println!("Storing key: {} with token: {}", id, token);
        Ok(Value::String(id))
    });

    // io.add_method("can_claim_recently_played_track", |params: Params| async {
    //     let (authorization, track_id, after, played_time) = 
    //         params.parse::<(String, String,u64, u8)>()
    //         .map_err(|e| Error::invalid_params(e.message))?;

    //     can_claim_recently_played_track(authorization,track_id,after,played_time).await
    //         .map(|result| Value::Bool(result))
    //         .map_err(|e| Error::invalid_params_with_details(e.to_string(), ""))
    // });

    io
}