use jsonrpc_http_server::{RequestMiddleware, RequestMiddlewareAction, hyper::{Body, Request}};

pub struct LoggerMiddleware;

impl RequestMiddleware for LoggerMiddleware {
    fn on_request(&self, request: Request<Body>) -> RequestMiddlewareAction {
        println!("Incoming request: {:?}", request.uri());
        println!("{:#?}", request);

        // Allow the request to proceed
        RequestMiddlewareAction::Proceed {
            should_continue_on_invalid_cors: false,
            request,
        }
    }
}