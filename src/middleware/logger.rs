use jsonrpc_http_server::{
    hyper::{Body, Request},
    RequestMiddleware, RequestMiddlewareAction,
};
use std::env;

pub struct LoggerMiddleware;

impl RequestMiddleware for LoggerMiddleware {
    fn on_request(&self, request: Request<Body>) -> RequestMiddlewareAction {
        let development = env::var("development").unwrap_or("false".to_string());
        if development != "true" {
            // println!("Incoming request: {:?}", request.uri());
            //println!("{:#?}", request);
        }

        // Allow the request to proceed
        RequestMiddlewareAction::Proceed {
            should_continue_on_invalid_cors: false,
            request,
        }
    }
}
