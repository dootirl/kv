use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use log::debug;
use tonic::{Request, Response, Status};

use key_value_store::key_value_store_server::KeyValueStore as KeyValueStoreService;
use key_value_store::{GetRequest, GetResponse, SetRequest, SetResponse};

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
