use std::error::Error;

use kv_store::{config::Config, run};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let config = Config::new();
    run(config).await?;

    Ok(())
}
