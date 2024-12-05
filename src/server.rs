use jsonrpc_core::{IoHandler,};
use jsonrpc_http_server::{ServerBuilder, DomainsValidation, AccessControlAllowOrigin};
use crate::middleware::logger::LoggerMiddleware;


pub fn create_server(io: IoHandler) {
    
    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Any, AccessControlAllowOrigin::Null]))
        .request_middleware(LoggerMiddleware)
        .threads(10)
        .start_http(&"0.0.0.0:3000".parse().unwrap())
        .unwrap();
    println!("Server started...");
    server.wait();
}