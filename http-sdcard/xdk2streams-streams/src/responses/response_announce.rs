use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseAnnounce {
    pub status: &'static str,
    pub channel_address: String,
    pub announcement_tag: String,
}
