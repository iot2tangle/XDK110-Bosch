use channels_lite::channels_lite::channel_author::Channel;
use local::security::keystore::KeyManager;
use local::stream_server;
use local::types::config::Config;

use std::{env, fs::File};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let author_key = &args[1];
    let subscriber_key = &args[2];

    KeyManager::new(author_key.to_string(), subscriber_key.to_string());

    let config: Config = serde_json::from_reader(File::open("config.json").unwrap()).unwrap();
    let endpoint = config.endpoint;
    let node = config.node;
    let mwm = config.mwm;
    let local_pow = config.local_pow;

    println!("Starting....");

    let c = Channel::new(node, mwm, local_pow, None);

    stream_server::start(endpoint, c).await
}
