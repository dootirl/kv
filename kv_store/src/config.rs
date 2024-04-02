use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(short, long, default_value = "[::1]:50051")]
    pub address: String,
    /*
     * #[arg(short, long)]
     * pub cert_path: String,
     *
     * #[arg(short, long)]
     * pub key_path: String,
     */
}

impl Config {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn new_empty() -> Self {
        Self::parse_from::<[_; 0], String>([])
    }
}
