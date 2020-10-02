use crate::is_valid;
use crate::security::keystore::{calculate_hash, KeyManager};
use crate::types::sensor_data::SensorData;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::iota_channels_lite::channel_author::Channel;
use crate::iota_channels_lite::utils::payload::json::PayloadBuilder;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

use hyper::{header, Body, Request, Response, StatusCode};

pub async fn sensor_data_response(
    req: Request<Body>,
    channel: Arc<Mutex<Channel>>,
    store: Arc<Mutex<KeyManager>>,
) -> Result<Response<Body>> {
    let data = hyper::body::to_bytes(req.into_body()).await?;

    let response;

    let json_data: serde_json::Result<SensorData> = serde_json::from_slice(&data);
    match json_data {
        Ok(mut data_ser) => {
            let hash = store
                .lock()
                .expect("lock keystore")
                .keystore
                .api_key_author
                .clone();
            if is_valid(&data_ser.device, hash.clone()) {
                data_ser.device.to_string().push_str("_id");
                data_ser.device = calculate_hash(data_ser.device);
                println!(
                    "POST /sensor_data -- {:?} -- authorized request by device",
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                );
                let mut channel = channel.lock().unwrap();
                //let message: String = serde_json::to_string(&data_ser).unwrap();
                match channel.write_signed(PayloadBuilder::new().public(&data_ser).unwrap().build())
                {
                    Ok(_) => {
                        response = Response::builder()
                            .status(StatusCode::OK)
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(Body::from("Data Sent sucessfully To Tangle"))?;
                    }
                    Err(_e) => {
                        response = Response::builder()
                            .status(500)
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(Body::from("Error while sending data"))?;
                    }
                };
            } else {
                response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from("Unauthorized"))?;
                println!(
                    "POST /sensor_data -- {:?} -- unauthorized request blocked",
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                );
            }
        }
        Err(_e) => {
            response = Response::builder()
                .status(500)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("Error while parsing input data"))?;
        }
    }
    Ok(response)
}
