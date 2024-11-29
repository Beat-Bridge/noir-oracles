use jsonrpc_http_server::ServerBuilder;
use jsonrpc_core::IoHandler;

pub fn create_server(io: IoHandler) {
    let server = ServerBuilder::new(io)
        .threads(10)
        .start_http(&"0.0.0.0:3000".parse().unwrap())
        .unwrap();
    println!("Server started...");
    server.wait();
}