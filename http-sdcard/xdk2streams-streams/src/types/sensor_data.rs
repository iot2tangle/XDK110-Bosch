use crate::types::sensor_type::SensorType;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorData {
    pub xdk2mam: Vec<SensorType>,
    pub device: String,
    pub timestamp: String,
}
