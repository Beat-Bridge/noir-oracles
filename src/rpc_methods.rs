use jsonrpc_core::{ params, IoHandler, Params, Error};
use serde_json::Value;

pub fn create_io() -> IoHandler {
    let mut io = IoHandler::default();
    

    io.add_method("can_claim_top_tracks", |params: Params| async {
        let parmeters  = params.parse::<(String,i8,i8)>();
        match parmeters {
            Ok(params) =>{
                Ok(Value::Bool(true))
            },
            Err(e) =>{
                Err(Error::invalid_params(e.message))
            }
        }
    });

    io.add_method("can_claim_top_artists", |_params: Params| async {
        Ok(Value::String(("Hello").to_string()))
    });

    io.add_method("can_claim_recently_played_tracks", |_params: Params| async {
        Ok(Value::String(("Hello").to_string()))
    });

    io
}