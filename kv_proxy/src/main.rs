use std::sync::Arc;

use axum::{
    extract::Query, extract::State, http::StatusCode, routing::get, routing::post, Json, Router,
};
use kv_store::key_value_store::key_value_store_client::KeyValueStoreClient;
use kv_store::key_value_store::{GetRequest, GetResponse, SetRequest, SetResponse};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tonic::transport::Channel;

#[tokio::main]
async fn main() {
    let client = Arc::new(Mutex::new(
        KeyValueStoreClient::connect("http://[::1]:50051")
            .await
            .unwrap(),
    ));

    let app = Router::new()
        .route("/get", get(kv_get))
        .with_state(client.clone())
        .route("/set", post(kv_set))
        .with_state(client.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct KvGetRequest {
    key: String,
}

#[derive(Serialize)]
struct KvGetResponse {
    value: Option<String>,
}

async fn kv_get(
    State(client): State<Arc<Mutex<KeyValueStoreClient<Channel>>>>,
    Query(params): Query<KvGetRequest>,
) -> (StatusCode, Json<KvGetResponse>) {
    let key = params.key.into();

    if key == "" {
        return (
            StatusCode::BAD_REQUEST,
            Json(KvGetResponse {
                value: Some("".to_string()),
            }),
        );
    }

    let request = tonic::Request::new(GetRequest { key });
    let GetResponse { value } = client.lock().await.get(request).await.unwrap().into_inner();
    (StatusCode::OK, Json(KvGetResponse { value }))
}

#[derive(Deserialize)]
struct KvSetRequest {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct KvSetResponse {}

async fn kv_set(
    State(client): State<Arc<Mutex<KeyValueStoreClient<Channel>>>>,
    Json(payload): Json<KvSetRequest>,
) -> (StatusCode, Json<KvSetResponse>) {
    let key = payload.key.into();
    let value = payload.value.into();

    if key == "" || value == "" {
        return (StatusCode::BAD_REQUEST, Json(KvSetResponse {}));
    }

    let request = tonic::Request::new(SetRequest { key, value });
    client.lock().await.set(request).await.unwrap().into_inner();
    (StatusCode::OK, Json(KvSetResponse {}))
}
