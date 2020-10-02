pub mod api;
pub mod iota_channels_lite;
pub mod responses;
pub mod security;
pub mod types;

use crate::iota_channels_lite::channel_author::Channel;

pub struct ChannelState {
    pub channel: Channel,
    pub channel_address: String,
    pub announcement_tag: String,
}

use crate::security::keystore::calculate_hash;
fn is_valid(key: &str, hash: String) -> bool {
    calculate_hash(key.to_string()) == hash
}
