extern crate serde_json;
pub struct ApiKeySubscriber(String);

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}
