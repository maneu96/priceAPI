/* ************************************************************************************************************************************/
// This is the main implementation of the API service
// It connects to a websocket url, gets and processes the data, while listening and responding to requests in the communication ports
/* ************************************************************************************************************************************/

mod api_responder;
mod binance_feed_processor;
mod websocket_feed;

use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    /******************************************************************************************************************* */
    // Declaration of variables and structs
    /******************************************************************************************************************* */

    // Declare Sender and Receiver so that information can be exchanged between threads
    let (sender, receiver) = mpsc::channel(128);
    // Declare the websocket url from which the feed will be extracted
    let url = "wss://stream.binance.com:9443/ws/btcusdt@ticker";
    // Declare structure that contains the websocket connection and the feed processing and information exchange methods
    let mut feed = websocket_feed::WebSocketFeed::new(sender, url).await;

    /******************************************************************************************************************* */
    //
    // Section 1: Secundary thread that gets the ws feed and processes it accordingly
    //
    /******************************************************************************************************************* */
    tokio::spawn(async move {
        let delay_retry = 20; // In case the ws connection fails, there will be an attempt of a new connection in 20 s
        loop {
            if let Err(e) = feed.connect_and_send().await {
                eprintln!("Error while getting WebSocket feed: {}, retrying in {} seconds", e, delay_retry);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(delay_retry)).await;
        }
    });

    /******************************************************************************************************************* */
    //
    // Section 2: Where the API listens to requests and deals with them accordingly
    //
    /******************************************************************************************************************* */

    let receiver = web::Data::new(Mutex::new(receiver)); // Encapsulate the receiver into a Mutex
    HttpServer::new(move || {
        App::new()
            .app_data(receiver.clone()) // Clone the Mutex receiver so that it can be accessed further on
            .route("/", web::get().to(api_responder::respond)) // Routes the request to the proper response handle
    })
    .bind("0.0.0.0:8080")? //Define the intended IP:Port where the communication will take place
    .run()
    .await
}
