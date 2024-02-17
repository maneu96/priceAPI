use futures_util::stream::StreamExt;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use native_tls::TlsConnector as NativeTlsConnector;

use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use serde_json::Value;
use tokio_tungstenite::{connect_async_tls_with_config, Connector};

use std::time::{SystemTime, UNIX_EPOCH};
use std::sync:: Mutex; //
//use std::sync::mpsc::{Sender, Receiver};

async fn get_feed(sender: mpsc::Sender<String>) -> Result<(), Box<dyn std::error::Error>>  {

    // URL of the Binance WebSocket for a specific trading pair (wss://stream.binance.com:9443/ws/*insert_pair*/)
    let url= "wss://stream.binance.com:9443/ws/btcusdt@ticker";
     // Convert the URL into a WebSocket request
    let request = url.into_client_request()?;
     // Create a new TlsConnector (from the native_tls crate)
    let tls_connector = NativeTlsConnector::new()?;
     // Convert it to the specific type thats required
    let connector = Connector::NativeTls(tls_connector);
     //Establish a WebSocket connection to the Binance Server
    // 'connect_async' returns a Future, which is awaited to get the socket connection and response.
    let (mut socket , _) = connect_async_tls_with_config(request,None, false, Some(connector)).await?;
    println!("Connected to the Server"); 
    // Listen for messages from the WebSocket Connection
    
    while let Some(msg) = socket.next().await{
        let msg = msg?;
        // if a message has no errors
        // Check if the message is text or binary (Expected types)
        if msg.is_text() || msg.is_binary(){
                //convert the message to text and unwrap it to get the string
                let data = msg.into_text()?;
                let data_json: Value = serde_json::from_str(&data)?; // convert the data string into JSON so that it is easier to Parse
                let binance_timestamp: u128 =   (data_json["E"].to_string()).parse()?;
                let latency = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()- binance_timestamp;
                let price = String::from("Latency (ms): ") + &latency.to_string()+ &String::from("\n") + &String::from("BTC Price (USDT): ") + data_json["c"].to_string().trim_matches('\"') + &String::from("\n");
                sender.send(price).await?;  // SEND THE data to the other Thread! x
        }
    }
    Ok(())
 }
 



async fn respond(receiver: web::Data<Mutex<mpsc::Receiver<String>>>) -> impl Responder{
   // Attempt to lock the receiver, handling the potential error without panicking
   let lock_result = receiver.lock();

   match lock_result {
       Ok(mut receiver) => {
           // Successfully acquired the lock, proceed to try receiving a message
           match receiver.try_recv() {
               Ok(message) => HttpResponse::Ok().body(message),
               Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve feed"),
           }
       },
       Err(e) => {
           // Handling the mutex poisoning case, or other errors in acquiring the lock
           eprintln!("Failed to lock the receiver: {:?}", e);
           HttpResponse::InternalServerError().body("Internal server error")
       }
   }
}
async fn echo(req_body: String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}


#[tokio::main]
async fn main() -> std::io::Result<()>{
    let (sender,receiver ) = mpsc::channel(128);

    tokio::spawn(async move {
        if let Err(e) = get_feed(sender).await {
            eprintln!("Error while getting WebSocket feed: {}", e);
        }
    });
    let receiver = web::Data::new(Mutex::new(receiver));
    
    HttpServer::new(move || {
        App::new()
            .app_data(receiver.clone())
            .route("/", web::get().to(respond))
            .route("/echo", web::post().to(echo))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await

}
