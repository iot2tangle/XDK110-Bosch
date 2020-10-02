use local::iota_channels_lite::channel_subscriber::Channel;
use local::iota_channels_lite::Network;
use local::types::sensor_data::SensorData;
use serde_json::Result;
use std::{env, thread, time};

pub struct Subscriber {
    channel_subscriber: Channel,
}

impl Subscriber {
    pub async fn new(channel_address: String, seed: Option<String>) -> Self {
        let (channel_address, announcement_tag) =
            (channel_address, "000000000000000000000000".to_string());
        let subscriber: Channel =
            Channel::new(Network::Main, channel_address, announcement_tag, seed);
        Self {
            channel_subscriber: subscriber,
        }
    }

    async fn read_all_public(&mut self) -> Result<Vec<String>> {
        let mut tag = self.channel_subscriber.get_next_message();

        let mut tag_list: Vec<String> = vec![];
        while tag.is_some() {
            println!("New Message with Id: {:?}", &tag.clone().unwrap());
            tag_list.push(tag.unwrap());
            tag = self.channel_subscriber.get_next_message();
        }

        let mut msg_list: Vec<String> = vec![];
        for signed_message_tag in tag_list {
            println!("Reading Message from Tangle...");
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
    let args: Vec<String> = env::args().collect();

    let channel_address = &args[1];

    let mut sub = Subscriber::new(channel_address.to_string(), None).await;

    sub.channel_subscriber.connect().unwrap();
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
