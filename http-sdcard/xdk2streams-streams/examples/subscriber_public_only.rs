use channels_lite::channels_lite::channel_subscriber::Channel;
use channels_lite::channels_lite::Network;
use local::types::sensor_data::SensorData;
use reqwest;
use serde_json::{Result, Value};
use std::{thread, time};

pub struct Subscriber {
    api_key: String,
    channel_subscriber: Channel,
}

async fn get_announcement(api_key: String) -> Result<(String, String)> {
    let client = reqwest::Client::new();

    let body = &client
        .get("http://localhost:8080/get_announcement")
        .header("x-api-key", api_key.clone())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .clone();

    let ret: Value = serde_json::from_str(body).unwrap();

    let channel_address = ret["channel_address"].as_str().unwrap().to_string();
    let announcement_tag = ret["announcement_tag"].as_str().unwrap().to_string();

    Ok((channel_address, announcement_tag))
}

impl Subscriber {
    pub async fn new(api_key: String, seed: Option<String>) -> Self {
        let (channel_address, announcement_tag) = get_announcement(api_key.clone()).await.unwrap();
        let subscriber: Channel =
            Channel::new(Network::Main, channel_address, announcement_tag, seed);
        Self {
            api_key: api_key.clone(),
            channel_subscriber: subscriber,
        }
    }

    async fn get_tags(&mut self) -> Result<Vec<String>> {
        let client = reqwest::Client::new();

        let body = &client
            .get("http://localhost:8080/get_tags")
            .header("x-api-key", self.api_key.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .clone();

        let mut tag_list: Vec<String> = vec![];
        if body != "" {
            let ret: Value = serde_json::from_str(body).unwrap();
            let list = { ret["signed_public"].as_array().unwrap().clone() };
            for t in &list {
                let signed_message_tag = t["signed_message_tag"].as_str().unwrap().to_string();
                tag_list.push(signed_message_tag);
            }
        }
        Ok(tag_list)
    }

    async fn read_all_public(&mut self) -> Result<Vec<String>> {
        let tag_list: Vec<String> = self.get_tags().await.unwrap();

        let mut msg_list: Vec<String> = vec![];

        for signed_message_tag in tag_list {
            let msgs: Vec<(Option<String>, Option<String>)> = self
                .channel_subscriber
                .read_signed(signed_message_tag)
                .unwrap();
            for (msg_p, _msg_m) in msgs {
                match msg_p {
                    None => continue,
                    Some(message) => msg_list.push(message),
                }
            }
        }

        Ok(msg_list)
    }
}

#[tokio::main]
async fn main() {
    let mut sub = Subscriber::new("SUB_KEY".to_string(), None).await;

    sub.channel_subscriber.connect(false).unwrap();
    println!("Connection to channel established successfully! \n Reading messages...");

    let public_list = sub.read_all_public().await.unwrap();

    for data in &public_list {
        let data: SensorData = serde_json::de::from_str(data).unwrap();
        println!("{:?}", data);
    }

    let mut public_list_len: usize = public_list.len();

    loop {
        let public_list = sub.read_all_public().await.unwrap();

        match public_list.last() {
            Some(last) => {
                if &public_list.len() != &public_list_len.clone() {
                    let data: SensorData = serde_json::de::from_str(&last).unwrap();
                    println!("{:?}", data);
                    public_list_len = public_list.len().clone();
                }
            }
            None => (),
        }

        thread::sleep(time::Duration::from_secs(2));
    }
}
