use std::env;
use std::net::SocketAddr;
use std::process::exit;

use log::{error, info};
use tonic::transport::Server;

use kv_store::key_value_store::key_value_store_server::KeyValueStoreServer;
use kv_store::KeyValueStore;

#[tokio::main]
async fn main() {
    env_logger::init();
    let addr: SocketAddr = env::var("LISTEN_ADDRESS")
        .unwrap_or("[::1]:50051".to_string())
        .parse()
        .unwrap_or_else(|_| {
            error!("Could not parse listen address");
            exit(1);
        });
    let store = KeyValueStore::new();

    info!("Listening on {}:{}", addr.ip(), addr.port());
    Server::builder()
        .add_service(KeyValueStoreServer::new(store))
        .serve(addr)
        .await
        .unwrap_or_else(|e| error!("{e}"));
}
