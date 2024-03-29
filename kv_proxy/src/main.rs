use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::response::{IntoResponse, Response};
use axum::{
    extract::Query, extract::State, http::StatusCode, routing::get, routing::post, Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use kv_store::key_value_store::key_value_store_client::KeyValueStoreClient;
use kv_store::key_value_store::{GetRequest, GetResponse, SetRequest};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::config::Config;
use crate::utils::exit_with_msg;

mod config;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = Config::build();
    let tls_config = RustlsConfig::from_pem_file(
        PathBuf::from(&config.cert_path),
        PathBuf::from(&config.key_path),
    )
    .await
    .unwrap_or_else(|_| exit_with_msg("Could not load certificates"));

    let client = Arc::new(Mutex::new(
        KeyValueStoreClient::connect(config.store_url.to_owned())
            .await
            .unwrap_or_else(|_| exit_with_msg("Could not connect to key-value store")),
    ));
    info!("Connected to key-value store on {}", config.store_url);

    let app = Router::new()
        .route("/get", get(store_get))
        .route("/set", post(store_set))
        .with_state(client.clone());

    let addr: SocketAddr = config
        .listen_address
        .parse()
        .unwrap_or_else(|_| exit_with_msg("Could not parse listen address"));

    info!("Listening on {}:{}", addr.ip(), addr.port());
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Deserialize)]
struct StoreGetRequest {
    key: String,
}

#[derive(Serialize)]
struct StoreGetResponse {
    value: Option<String>,
}

async fn store_get(
    State(client): State<Arc<Mutex<KeyValueStoreClient<Channel>>>>,
    Query(params): Query<StoreGetRequest>,
) -> Response {
    let key = params.key.into();

    if key == "" {
        debug!("Get - key: `{key}` - invalid request (empty key)");
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "`key` argument must be provided".to_string(),
            }),
        )
            .into_response();
    }
    debug!("Get - key: `{key}`");

    let request = tonic::Request::new(GetRequest { key });
    let GetResponse { value } = client.lock().await.get(request).await.unwrap().into_inner();

    (StatusCode::OK, Json(StoreGetResponse { value })).into_response()
}

#[derive(Deserialize)]
struct StoreSetRequest {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct StoreSetResponse {}

async fn store_set(
    State(client): State<Arc<Mutex<KeyValueStoreClient<Channel>>>>,
    Json(payload): Json<StoreSetRequest>,
) -> Response {
    let key = payload.key.into();
    let value = payload.value.into();

    if key == "" || value == "" {
        debug!("Set - key: `{key}`, value: `{value}` - invalid request (empty key/value)");
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "`key` and `value` arguments must be provided".to_string(),
            }),
        )
            .into_response();
    }
    debug!("Set - key: `{key}`, value: `{value}`");

    let request = tonic::Request::new(SetRequest { key, value });
    client.lock().await.set(request).await.unwrap().into_inner();

    (StatusCode::OK, Json(StoreSetResponse {})).into_response()
}
