//!
//! Channel Subscriber
//!
use super::Network;
use crate::utils::{payload::json::Payload, random_seed};
use iota::client as iota_client;
use iota_streams::app::transport::tangle::{
    client::{RecvOptions, SendTrytesOptions},
    PAYLOAD_BYTES,
};
use iota_streams::app::transport::Transport;
use iota_streams::app_channels::{
    api::tangle::{Address, Subscriber},
    message,
};

use anyhow::Result;

///
/// Channel subscriber
///
pub struct Channel {
    subscriber: Subscriber,
    is_connected: bool,
    send_opt: SendTrytesOptions,
    announcement_link: Address,
    subscription_link: Address,
    channel_address: String,
}

impl Channel {
    ///
    /// Initialize the subscriber
    ///
    pub fn new(
        node: Network,
        channel_address: String,
        announcement_tag: String,
        seed_option: Option<String>,
    ) -> Channel {
        let seed = match seed_option {
            Some(seed) => seed,
            None => random_seed::new(),
        };
        let subscriber = Subscriber::new(&seed, "utf-8", PAYLOAD_BYTES);
        iota_client::Client::add_node(node.as_string()).unwrap();

        Self {
            subscriber: subscriber,
            is_connected: false,
            send_opt: node.send_options(),
            announcement_link: Address::from_str(&channel_address, &announcement_tag).unwrap(),
            subscription_link: Address::default(),
            channel_address: channel_address,
        }
    }

    ///
    /// Connect
    ///
    pub fn connect(&mut self, masked: bool) -> Result<String> {
        let message_list = iota_client::Client::get()
            .recv_messages_with_options(&self.announcement_link, RecvOptions::default())?;

        let mut found_valid_msg = false;

        for tx in message_list.iter() {
            let header = tx.parse_header()?;
            if header.check_content_type(message::ANNOUNCE) {
                self.subscriber.unwrap_announcement(header.clone())?;
                found_valid_msg = true;
                break;
            }
        }
        if masked {
            if found_valid_msg {
                let subscribe_link = {
                    let msg = self.subscriber.subscribe(&self.announcement_link)?;
                    iota_client::Client::get().send_message_with_options(&msg, self.send_opt)?;
                    msg.link.clone()
                };

                self.subscription_link = subscribe_link;
                self.is_connected = true;
            } else {
                println!("No valid announce message found");
            }
            Ok(self.subscription_link.msgid.to_string())
        } else {
            Ok("Connected in public mode".to_string())
        }
    }

    /*
    ///
    /// Disconnect
    ///
    pub fn disconnect(&mut self) -> Result<String> {
        let unsubscribe_link = {
            let msg = self.subscriber.unsubscribe(&self.subscription_link)?;
            iota_client::Client::get().send_message_with_options(&msg, self.send_opt)?;
            msg.link.msgid
        };
        Ok(unsubscribe_link.to_string())
    }*/

    ///
    /// Read signed packet
    ///
    pub fn read_signed(
        &mut self,
        signed_packet_tag: String,
    ) -> Result<Vec<(Option<String>, Option<String>)>> {
        let mut response: Vec<(Option<String>, Option<String>)> = Vec::new();

        let link = Address::from_str(&self.channel_address, &signed_packet_tag).unwrap();
        let message_list =
            iota_client::Client::get().recv_messages_with_options(&link, RecvOptions::default())?;

        for tx in message_list.iter() {
            let header = tx.parse_header()?;
            if header.check_content_type(message::SIGNED_PACKET) {
                match self.subscriber.unwrap_signed_packet(header.clone()) {
                    Ok((_signer, unwrapped_public, unwrapped_masked)) => {
                        response.push((
                            Payload::unwrap_data(&String::from_utf8(unwrapped_public.0).unwrap())
                                .unwrap(),
                            Payload::unwrap_data(&String::from_utf8(unwrapped_masked.0).unwrap())
                                .unwrap(),
                        ));
                    }
                    Err(_) => (), //println!("Signed Packet Error: {}", e),
                }
            }
        }

        Ok(response)
    }

    ///
    /// Read tagged packet
    ///
    pub fn read_tagged(
        &mut self,
        tagged_packet_tag: String,
    ) -> Result<Vec<(Option<String>, Option<String>)>> {
        let mut response: Vec<(Option<String>, Option<String>)> = Vec::new();

        if self.is_connected {
            let link = Address::from_str(&self.channel_address, &tagged_packet_tag).unwrap();

            let message_list = iota_client::Client::get()
                .recv_messages_with_options(&link, RecvOptions::default())?;

            for tx in message_list.iter() {
                let header = tx.parse_header()?;
                if header.check_content_type(message::TAGGED_PACKET) {
                    match self.subscriber.unwrap_tagged_packet(header.clone()) {
                        Ok((unwrapped_public, unwrapped_masked)) => {
                            response.push((
                                Payload::unwrap_data(
                                    &String::from_utf8(unwrapped_public.0).unwrap(),
                                )
                                .unwrap(),
                                Payload::unwrap_data(
                                    &String::from_utf8(unwrapped_masked.0).unwrap(),
                                )
                                .unwrap(),
                            ));
                        }
                        Err(e) => println!("Tagged Packet Error: {}", e),
                    }
                }
            }
        } else {
            println!("Channel not connected");
        }

        Ok(response)
    }

    ///
    /// Update keyload
    ///
    pub fn update_keyload(&mut self, keyload_tag: String) -> Result<()> {
        let keyload_link = Address::from_str(&self.channel_address, &keyload_tag).unwrap();

        if self.is_connected {
            let message_list = iota_client::Client::get()
                .recv_messages_with_options(&keyload_link, RecvOptions::default())?;

            for tx in message_list.iter() {
                let header = tx.parse_header()?;
                if header.check_content_type(message::KEYLOAD) {
                    match self.subscriber.unwrap_keyload(header.clone()) {
                        Ok(_) => {
                            break;
                        }
                        Err(e) => println!("Keyload Packet Error: {}", e),
                    }
                } else {
                    println!(
                        "Expected a keyload message, found {}",
                        header.content_type()
                    );
                }
            }
        }

        Ok(())
    }
}
