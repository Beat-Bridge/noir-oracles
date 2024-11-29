use rpc_methods::create_io;
use server::create_server;

pub mod  query_builder;
pub mod types;
pub mod server;
pub mod rpc_methods;


#[tokio::main]
async fn main() {
    let io = create_io();
    create_server(io);
}