use dotenv::dotenv;
use rpc_methods::create_io;
use server::create_server;

pub mod middleware;
pub mod query_builder;
pub mod redis;
pub mod rpc_methods;
pub mod server;
pub mod types;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let io = create_io();
    create_server(io);
}
