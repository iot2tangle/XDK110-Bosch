extern crate channels_lite;
pub mod api;
pub mod responses;
pub mod security;
pub mod stream_server;
pub mod types;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use channels_lite::channels_lite::channel_author::Channel;

pub struct ChannelState {
    pub channel: Channel,
    pub channel_address: String,
    pub announcement_tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnnouncementInfo {
    pub channel_address: String,
    pub announcement_tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedTags {
    pub signed_message_tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagLists {
    pub signed_public: Vec<SignedTags>,
    pub signed_masked: Vec<SignedTags>,
}

use crate::security::keystore::calculate_hash;
fn is_valid(key: &str, hash: String) -> bool {
    calculate_hash(key.to_string()) == hash
}
