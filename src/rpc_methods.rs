use jsonrpc_core::{ IoHandler, Params};
use serde_json::Value;

pub fn create_io() -> IoHandler {
    let mut io = IoHandler::default();
    
    io.add_method("can_claim_top_tracks", |_params: Params| async {
        Ok(Value::String(("Hello").to_string()))
    });

    io.add_method("can_claim_top_artists", |_params: Params| async {
        Ok(Value::String(("Hello").to_string()))
    });

    io.add_method("can_claim_recently_played_tracks", |_params: Params| async {
        Ok(Value::String(("Hello").to_string()))
    });

    io
}