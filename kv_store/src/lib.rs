use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use config::Config;
use log::{debug, error, info};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use key_value_store::key_value_store_server::{
    KeyValueStore as KeyValueStoreService, KeyValueStoreServer,
};

use key_value_store::{GetRequest, GetResponse, SetRequest, SetResponse};

pub mod config;
pub mod key_value_store {
    tonic::include_proto!("key_value_store");
}

pub struct KeyValueStore {
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl KeyValueStoreService for KeyValueStore {
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let store = self.store.lock().unwrap();
        let GetRequest { key } = request.into_inner();

        let reply = key_value_store::GetResponse {
            value: store.get(&key).cloned(),
        };

        debug!("Get - key: `{key}`");

        Ok(Response::new(reply))
    }

    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let mut store = self.store.lock().unwrap();
        let SetRequest { key, value } = request.into_inner();

        store.insert(key.clone(), value);
        let reply = key_value_store::SetResponse {};

        debug!("Set - key: `{key}`, value: `value`");

        Ok(Response::new(reply))
    }
}

pub async fn run(config: Config) -> Result<(), String> {
    let store = KeyValueStore::new();
    let addr: SocketAddr = config
        .address
        .parse()
        .map_err(|_| format!("Could not parse `{}`", config.address))?;

    info!("Listening on {}:{}", addr.ip(), addr.port());
    Server::builder()
        .add_service(KeyValueStoreServer::new(store))
        .serve(addr)
        .await
        .unwrap_or_else(|e| error!("{e}"));

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use tokio::time::{sleep, Duration};

    use crate::config::Config;
    use crate::key_value_store::key_value_store_client::KeyValueStoreClient;
    use crate::{run, GetRequest, GetResponse, SetRequest};

    #[test]
    fn default_config() {
        let config = Config::new_empty();
        assert_eq!(config.address, "[::1]:50051");
    }

    #[tokio::test]
    async fn empty_get() {
        let config = Config::new_empty();
        let addr: SocketAddr = config.address.parse().unwrap();
        let task = tokio::spawn(async move { run(config).await });
        sleep(Duration::from_secs(2)).await;
        let mut client = KeyValueStoreClient::connect(format!("http://{}", addr))
            .await
            .unwrap();

        let request = tonic::Request::new(GetRequest {
            key: "foo".to_owned(),
        });

        let response = client.get(request).await.unwrap();

        assert_eq!(response.into_inner(), GetResponse { value: None });
        task.abort()
    }

    #[tokio::test]
    async fn nonempty_get() {
        let config = Config::new_empty();
        let addr: SocketAddr = config.address.parse().unwrap();
        let task = tokio::spawn(async move { run(config).await });
        sleep(Duration::from_secs(2)).await;
        let mut client = KeyValueStoreClient::connect(format!("http://{}", addr))
            .await
            .unwrap();

        let set_request = tonic::Request::new(SetRequest {
            key: "bar".to_owned(),
            value: "baz".to_owned(),
        });
        client.set(set_request).await.unwrap();

        sleep(Duration::from_secs(2)).await;

        let get_request = tonic::Request::new(GetRequest {
            key: "bar".to_owned(),
        });
        let get_response = client.get(get_request).await.unwrap();

        assert_eq!(
            get_response.into_inner(),
            GetResponse {
                value: Some("baz".to_owned())
            }
        );
        task.abort();
    }
}
