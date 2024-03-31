use kv_store::{get_addr, run};

#[tokio::main]
async fn main() {
    env_logger::init();
    run(get_addr()).await;
}
