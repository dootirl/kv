use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(short, long, default_value = "127.0.0.1:3000")]
    pub address: String,

    #[arg(short, long, default_value = "certs/cert.pem")]
    pub cert_path: String,

    #[arg(short, long, default_value = "certs/key.pem")]
    pub key_path: String,

    #[arg(short, long, default_value = "https://[::1]:50051")]
    pub store_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn new_empty() -> Self {
        Self::parse_from::<[_; 0], String>([])
    }
}
