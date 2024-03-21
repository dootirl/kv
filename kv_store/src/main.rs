use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tonic::{transport::Server, Request, Response, Status};

use kv_store::key_value_store::key_value_store_server::{
    KeyValueStore as KeyValueStoreService, KeyValueStoreServer,
};
use kv_store::key_value_store::{self, GetRequest, GetResponse, SetRequest, SetResponse};

struct KeyValueStore {
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl KeyValueStore {
    fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl KeyValueStoreService for KeyValueStore {
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let store = self.store.lock().unwrap();
        let key = request.into_inner().key;

        let reply = key_value_store::GetResponse {
            value: store.get(&key).cloned(),
        };

        Ok(Response::new(reply))
    }

    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        println!("in");
        let mut store = self.store.lock().unwrap();
        let SetRequest { key, value } = request.into_inner();

        println!("inserting {} {}", key, value);
        store.insert(key, value);
        let reply = key_value_store::SetResponse {};
        println!("inserted!");

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let store = KeyValueStore::new();

    Server::builder()
        .add_service(KeyValueStoreServer::new(store))
        .serve(addr)
        .await?;

    Ok(())
}
