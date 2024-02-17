mod websocket_feed;
mod binance_feed_processor;
mod api_responder;

use actix_web::{web, App, HttpServer};
use tokio::sync::mpsc;
use std::sync::Mutex;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //
    //
    // Declaration of variables and structs
    // 
    //

    // Declare Sender and Receiver so that information can be exchanged between threads
    let (sender, receiver) = mpsc::channel(128); 
    // Declare the websocket url from which the feed will be extracted
    let url = "wss://stream.binance.com:9443/ws/btcusdt@ticker";
    // Declare structure that contains the websocket connection and the feed processing and information exchange methods
    let mut feed = websocket_feed::WebSocketFeed::new(sender, url).await;
   
    //
    //
    // Section 1: Secundary thread that gets the ws feed and processes it accordingly
    //
    // 
    tokio::spawn(async move {
        if let Err(e) = feed.connect_and_send().await {
            eprintln!("Error while getting WebSocket feed: {}", e);
        }
    });
   
    //
    //
    // Section 2: Where the API listens to requests and deals with them accordingly
    //
    //
   
    let receiver = web::Data::new(Mutex::new(receiver));
    HttpServer::new(move || {
        App::new()
            .app_data(receiver.clone())
            .route("/", web::get().to(api_responder::respond))
            .route("/echo", web::post().to(api_responder::echo))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}