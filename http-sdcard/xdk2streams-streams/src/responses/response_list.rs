use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseList {
    pub status: &'static str,
    pub list: Vec<String>,
}
