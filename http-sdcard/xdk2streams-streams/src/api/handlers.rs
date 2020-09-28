use crate::is_valid;
use crate::responses::response_message::ResponseMessage;
use crate::security::keystore::{calculate_hash, KeyManager};
use crate::types::sensor_data::SensorData;
use crate::AnnouncementInfo;
use crate::ChannelState;
use crate::SignedTags;
use crate::TagLists;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use channels_lite::utils::payload::json::PayloadBuilder;

pub async fn status() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(format!("OK")))
}

pub async fn get_announcement(
    req: HttpRequest,
    store: web::Data<KeyManager>,
    channel: web::Data<Arc<Mutex<ChannelState>>>,
) -> Result<HttpResponse, Error> {
    let hash = store.keystore.api_key_subscriber.clone();
    if is_valid(
        req.headers().get("x-api-key").unwrap().to_str().unwrap(),
        hash.clone(),
    ) {
        println!(
            "GET /get_announcement -- {:?} -- authorized request by subscriber",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        let channel = channel.lock().unwrap();
        let channel_address = channel.channel_address.clone();
        let announcement_tag = channel.announcement_tag.clone();
        Ok(HttpResponse::Ok().json(AnnouncementInfo {
            channel_address: channel_address,
            announcement_tag: announcement_tag,
        }))
    } else {
        println!(
            "GET /get_announcement -- {:?} -- unauthorized request blocked",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        Ok(HttpResponse::Unauthorized().json("Unauthorized"))
    }
}

pub async fn get_tags(
    tag_lists: web::Data<Arc<Mutex<TagLists>>>,
    req: HttpRequest,
    store: web::Data<KeyManager>,
) -> Result<HttpResponse, Error> {
    let hash = store.keystore.api_key_subscriber.clone();
    if is_valid(
        req.headers().get("x-api-key").unwrap().to_str().unwrap(),
        hash.clone(),
    ) {
        /*
        println!(
            "GET /tag_lists -- {:?} -- authorized request by subscriber",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        */
        let tag_lists = tag_lists.lock().unwrap();
        Ok(HttpResponse::Ok().json(tag_lists.clone()))
    } else {
        println!(
            "GET /tag_lists -- {:?} -- unauthorized request blocked",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        Ok(HttpResponse::Unauthorized().json("Unauthorized"))
    }
}

pub async fn add_subscriber(
    data: Option<String>,
    req: HttpRequest,
    store: web::Data<KeyManager>,
    channel: web::Data<Arc<Mutex<ChannelState>>>,
) -> Result<HttpResponse, Error> {
    let hash = store.keystore.api_key_subscriber.clone();
    if is_valid(
        req.headers().get("x-api-key").unwrap().to_str().unwrap(),
        hash.clone(),
    ) {
        println!(
            "PUT /add_subscriber -- {:?} -- authorized request by subscriber",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        match data {
            Some(data) => {
                let mut channel = channel.lock().unwrap();
                match channel.channel.add_subscriber(data) {
                    Ok(keyload) => {
                        Ok(HttpResponse::Ok().json(ResponseMessage { message: keyload }))
                    }
                    Err(e) => Ok(HttpResponse::Ok().json(format!("{}", e))),
                }
            }
            None => Ok(HttpResponse::Ok().json(format!("No thing!"))),
        }
    } else {
        println!(
            "PUT /add_subscriber -- {:?} -- unauthorized request blocked",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        Ok(HttpResponse::Unauthorized().json("Unauthorized"))
    }
}

pub async fn sensor_data_public(
    data: Option<String>,
    store: web::Data<KeyManager>,
    channel: web::Data<Arc<Mutex<ChannelState>>>,
    list: web::Data<Arc<Mutex<TagLists>>>,
) -> Result<HttpResponse, Error> {
    match data {
        Some(data) => {
            let json_data: serde_json::Result<SensorData> = serde_json::from_str(&data);
            match json_data {
                Ok(mut data_ser) => {
                    let hash = store.keystore.api_key_author.clone();
                    if is_valid(&data_ser.device, hash.clone()) {
                        data_ser.device.to_string().push_str("_id");
                        data_ser.device = calculate_hash(data_ser.device);
                        println!(
                            "POST /sensor_data_public -- {:?} -- authorized request by device",
                            SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
                        );
                        let mut channel = channel.lock().unwrap();
                        //let message: String = serde_json::to_string(&data_ser).unwrap();
                        match channel.channel.write_signed(
                            false,
                            PayloadBuilder::new().public(&data_ser).unwrap().build(),
                        ) {
                            Ok(message_tag) => {
                                list.lock().expect("lock list data").signed_public.push(
                                    SignedTags {
                                        signed_message_tag: message_tag.clone(),
                                    },
                                );

                                Ok(HttpResponse::Ok()
                                    .json(format!("Data Successfully sent to Tangle!")))
                            }
                            Err(_e) => Ok(HttpResponse::Ok().json(format!("Error sending data"))),
                        }
                    } else {
                        println!(
                            "POST /sensor_data_public -- {:?} -- unauthorized request blocked",
                            SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
                        );
                        Ok(HttpResponse::Unauthorized().json("Unauthorized"))
                    }
                }
                Err(_e) => Ok(HttpResponse::Ok().json(format!("Error getting data"))),
            }
        }
        None => Ok(HttpResponse::Ok().json(format!("No thing!"))),
    }
}

pub async fn sensor_data_masked(
    data: Option<String>,
    store: web::Data<KeyManager>,
    channel: web::Data<Arc<Mutex<ChannelState>>>,
    list: web::Data<Arc<Mutex<TagLists>>>,
) -> Result<HttpResponse, Error> {
    match data {
        Some(data) => {
            let json_data: serde_json::Result<SensorData> = serde_json::from_str(&data);
            match json_data {
                Ok(mut data_ser) => {
                    let hash = store.keystore.api_key_author.clone();
                    if is_valid(&data_ser.device, hash.clone()) {
                        data_ser.device.to_string().push_str("_id");
                        data_ser.device = calculate_hash(data_ser.device);
                        println!(
                            "POST /sensor_data_masked -- {:?} -- authorized request by device",
                            SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
                        );

                        let mut channel = channel.lock().unwrap();
                        //let message: String = serde_json::to_string(&data_ser).unwrap();
                        if channel.channel.can_send_masked() {
                            match channel.channel.write_signed(
                                true,
                                PayloadBuilder::new().masked(&data_ser).unwrap().build(),
                            ) {
                                Ok(message_tag) => {
                                    list.lock().expect("lock list data").signed_masked.push(
                                        SignedTags {
                                            signed_message_tag: message_tag.clone(),
                                        },
                                    );

                                    Ok(HttpResponse::Ok()
                                        .json(format!("Data Successfully sent to Tangle!")))
                                }
                                Err(_e) => Ok(HttpResponse::Ok().json(format!("ERROR"))),
                            }
                        } else {
                            Ok(HttpResponse::Unauthorized()
                                .json("Action invalid: No encryption key generated by subscribers"))
                        }
                    } else {
                        println!(
                            "POST /sensor_data_masked -- {:?} -- unauthorized request blocked",
                            SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
                        );
                        Ok(HttpResponse::Unauthorized().json("Unauthorized"))
                    }
                }
                Err(_e) => Ok(HttpResponse::Ok().json(format!("ERROR"))),
            }
        }
        None => Ok(HttpResponse::Ok().json(format!("No thing!"))),
    }
}
