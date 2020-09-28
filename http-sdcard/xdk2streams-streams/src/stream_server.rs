use actix_web::{guard, web, App, HttpResponse, HttpServer};

use crate::api::handlers;

use crate::security::keystore::KeyManager;

use channels_lite::channels_lite::channel_author::Channel;

use crate::ChannelState;
use crate::TagLists;

use std::sync::{Arc, Mutex};

pub async fn start(endpoint: String, mut channel: Channel) -> std::io::Result<()> {
    let (x, y) = channel.open().unwrap();

    println!("Opened Streams channel at address:\n {}", &x);
    let channel_state = Arc::new(Mutex::new(ChannelState {
        channel: channel,
        channel_address: x,
        announcement_tag: y,
    }));

    let tag_store = Arc::new(Mutex::new(TagLists {
        signed_public: vec![],
        signed_masked: vec![],
    }));

    println!("Started server at: {}", &endpoint);
    HttpServer::new(move || {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .data(KeyManager::restore())
            .data(channel_state.clone())
            .data(tag_store.clone())
            .service(web::resource("/status").route(web::get().to(handlers::status)))
            .service(
                web::resource("/sensor_data_public")
                    .route(web::post().to(handlers::sensor_data_public)),
            )
            .service(
                web::resource("/sensor_data_masked")
                    .route(web::post().to(handlers::sensor_data_masked)),
            )
            .service(
                web::resource("/get_announcement")
                    .route(web::get().to(handlers::get_announcement))
                    .guard(guard::fn_guard(|req| {
                        req.headers().contains_key("x-api-key")
                    }))
                    .to(|| HttpResponse::MethodNotAllowed()),
            )
            .service(
                web::resource("/get_tags")
                    .route(web::get().to(handlers::get_tags))
                    .guard(guard::fn_guard(|req| {
                        req.headers().contains_key("x-api-key")
                    }))
                    .to(|| HttpResponse::MethodNotAllowed()),
            )
            .service(
                web::resource("/add_subscriber")
                    .route(web::put().to(handlers::add_subscriber))
                    .guard(guard::fn_guard(|req| {
                        req.headers().contains_key("x-api-key")
                    }))
                    .to(|| HttpResponse::MethodNotAllowed()),
            )
    })
    .bind(endpoint)?
    .run()
    .await
}
