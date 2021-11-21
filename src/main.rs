#![allow(dead_code)]
use net::Server;
use website_handler::WebsiteHandler;
use std::env;

mod http;
mod net;
mod website_handler;

fn main() {
    let public_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path.to_string()));
}