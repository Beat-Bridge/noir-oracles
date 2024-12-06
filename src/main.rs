use rpc_methods::create_io;
use server::create_server;
use dotenv::dotenv;

pub mod  query_builder;
pub mod types;
pub mod server;
pub mod rpc_methods;
pub mod middleware;
pub mod  redis;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let io = create_io();
    create_server(io);
}