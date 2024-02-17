use futures_util::StreamExt;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::client::IntoClientRequest, Connector};
use native_tls::TlsConnector as NativeTlsConnector;

use crate::binance_feed_processor::BinanceFeedProcessor; // import the binance feed processor module

pub struct WebSocketFeed {
    sender: Sender<String>,
    url: String,
}

impl WebSocketFeed {
    pub async fn new(sender: Sender<String>, url: &str) -> Self {
        WebSocketFeed {
            sender,
            url: url.to_string(),
        }
    }

    pub async fn connect_and_send(&mut self) -> Result<(), Box<dyn std::error::Error>> {
   
    // Convert the URL into a WebSocket request
   let request = self.url.clone().into_client_request()?;
    // Create a new TlsConnector (from the native_tls crate)
   let tls_connector = NativeTlsConnector::new()?;
    // Convert it to the specific type thats required
   let connector = Connector::NativeTls(tls_connector);
    //Establish a WebSocket connection to the Binance Server
   // 'connect_async....' returns a Future, which is awaited to get the socket connection and response, or an error message.
   let (mut socket , _) = connect_async_tls_with_config(request,None, false, Some(connector)).await?;
   println!("Connected to the WebSocket"); 
   // Listen for messages from the WebSocket Connection
   
   while let Some(msg) = socket.next().await{
       let msg = msg?;
       // If a message has no errors
       // Check if the message is text or binary (Expected types)
       if msg.is_text() || msg.is_binary(){
            let data = msg.into_text()?; // CONVERT the message to text and get the string
            let processed_data = BinanceFeedProcessor::process(&data)?;
            self.sender.send(processed_data).await?;  // SEND data to the other thread! 
       }
   }
   Ok(())
}
}