//!
//! Channel author
//!
use crate::utils::{payload::PacketPayload, random_seed};
use anyhow::{bail, Result};
use iota::client as iota_client;
use iota_streams::app::transport::tangle::{
    client::{RecvOptions, SendTrytesOptions},
    PAYLOAD_BYTES,
};
use iota_streams::app::transport::Transport;
use iota_streams::app_channels::{
    api::tangle::{Address, Author},
    message,
};
use std::string::ToString;

///
/// Channel
///
pub struct Channel {
    author: Author,
    send_opt: SendTrytesOptions,
    channel_address: String,
    announcement_id: String,
    last_keyload_tag: String,
}

impl Channel {
    ///
    /// Initialize the Channel
    ///
    pub fn new(node: String, mwm: u8, local_pow: bool, seed_option: Option<String>) -> Channel {
        let seed = match seed_option {
            Some(seed) => seed,
            None => random_seed::new(),
        };

        let author = Author::new(&seed, "utf-8", PAYLOAD_BYTES, false);
        iota_client::Client::add_node(&node).unwrap();

        let channel_address = author.channel_address().unwrap().to_string();

        let mut send_opt = SendTrytesOptions::default();
        send_opt.min_weight_magnitude = mwm;
        send_opt.local_pow = local_pow;

        Self {
            author: author,
            send_opt: send_opt,
            channel_address: channel_address,
            announcement_id: String::default(),
            last_keyload_tag: String::default(),
        }
    }

    ///
    /// Open a channel
    ///
    pub fn open(&mut self) -> Result<(String, String)> {
        let announcement_message = self.author.announce()?;
        iota_client::Client::get()
            .send_message_with_options(&announcement_message, self.send_opt)?;

        self.announcement_id = announcement_message.link.msgid.to_string();

        Ok((self.channel_address.clone(), self.announcement_id.clone()))
    }

    ///
    /// Add subscriber
    ///
    pub fn add_subscriber(&mut self, subscribe_tag: String) -> Result<String> {
        let subscribe_link = match Address::from_str(&self.channel_address, &subscribe_tag) {
            Ok(subscribe_link) => subscribe_link,
            Err(()) => bail!(
                "Failed to create Address from {}:{}",
                &self.channel_address,
                &subscribe_tag
            ),
        };

        let message_list = iota_client::Client::get()
            .recv_messages_with_options(&subscribe_link, RecvOptions::default())?;
        for tx in message_list.iter() {
            let header = tx.parse_header()?;
            if header.check_content_type(message::SUBSCRIBE) {
                match self.author.unwrap_subscribe(header.clone()) {
                    Ok(_) => {
                        break;
                    }
                    Err(e) => println!("Subscribe Packet Error: {}", e),
                }
            }
        }

        self.last_keyload_tag = {
            let keyload = self.author.share_keyload_for_everyone(&subscribe_link)?;
            iota_client::Client::get().send_message_with_options(&keyload.0, self.send_opt)?;
            keyload.0.link.msgid.to_string()
        };

        Ok(self.last_keyload_tag.clone())
    }

    ///
    /// Write signed packet
    ///
    pub fn write_signed<T>(&mut self, masked: bool, payload: T) -> Result<String>
    where
        T: PacketPayload,
    {
        let signed_packet_link = {
            if masked {
                let keyload_link =
                    Address::from_str(&self.channel_address, &self.last_keyload_tag).unwrap();
                let msg = self.author.sign_packet(
                    &keyload_link,
                    &payload.public_data(),
                    &payload.masked_data(),
                )?;
                let ret_link = msg.0;
                iota_client::Client::get().send_message_with_options(&ret_link, self.send_opt)?;
                ret_link.link.clone()
            } else {
                let msg = self.author.sign_packet(
                    &Address::from_str(&self.channel_address, &self.announcement_id).unwrap(),
                    &payload.public_data(),
                    &payload.masked_data(),
                )?;
                let ret_link = msg.0;
                iota_client::Client::get().send_message_with_options(&ret_link, self.send_opt)?;
                ret_link.link.clone()
            }
        };

        Ok(signed_packet_link.msgid.to_string())
    }

    ///
    /// Write tagged packet
    ///
    pub fn write_tagged<T>(&mut self, payload: T) -> Result<String>
    where
        T: PacketPayload,
    {
        let keyload_link =
            Address::from_str(&self.channel_address, &self.last_keyload_tag).unwrap();

        let tagged_packet_link = {
            let msg = self.author.tag_packet(
                &keyload_link,
                &payload.public_data(),
                &payload.masked_data(),
            )?;
            let ret_link = msg.0;
            iota_client::Client::get().send_message_with_options(&ret_link, self.send_opt)?;
            ret_link.link.clone()
        };

        Ok(tagged_packet_link.msgid.to_string())
    }

    ///
    /// Returns wether the Author can send a masked message
    ///
    pub fn can_send_masked(&mut self) -> bool {
        self.last_keyload_tag != String::default()
    }

    /*
    ///
    /// Remove subscriber
    ///
    ///
    pub fn remove_subscriber(&mut self, unsubscribe_tag: String) -> Result<()> {
        let unsubscribe_link = Address::from_str(&self.channel_address, &unsubscribe_tag).unwrap();

        let message_list = iota_client::Client::get()
            .recv_messages_with_options(&unsubscribe_link, RecvOptions::default())?;
        for tx in message_list.iter() {
            let header = tx.parse_header()?;
            if header.check_content_type(message::UNSUBSCRIBE) {
                match self.author.unsubscribe(header.clone()) {
                    Ok(_) => {
                        break;
                    }
                    Err(e) => println!("Unsubscribe Packet Error: {}", e),
                }
            }
        }
        Ok(())
    }
    */
}
