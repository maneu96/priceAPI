/* ************************************************************************************************************************************/
// This is an implementation of an API responder module
// It is called by the main, whenever a response to a client request is necessary.
// respond returns either a Responder with a string with an HTTP message or an error and it calls the module BinanceFeedProcessor to 
// handle the raw data . it is built in a way that could be expandable for other structures of raw data.(other exchanges, for instance)
/* ************************************************************************************************************************************/
use actix_web::{web, HttpResponse, Responder};
use tokio::sync::watch::Receiver;

use crate::binance_feed_processor::BinanceFeedProcessor;

pub async fn respond( receiver: web::Data<Receiver<String>>) -> impl Responder {
    /***********************************************************************************************************************/
    //
    //  inputs :
    //          receiver: web::Data<Receiver<String>> // A Receiver encapsulated by the Data structure of the API.
    //                                                  It can access the string of the message to be transmitted to the client
    // outputs:
    //           impl Responder // An implied Responder that contains an HTTP response or an error message
    /***********************************************************************************************************************/
    // `borrow_and_update` or `changed().await` can be used here depending on the use case.
    // `borrow` gives us access to the latest value sent through the channel without waiting for a new value.
    // If you want to wait for the next value, use `receiver.changed().await` instead.

    let message = receiver.borrow().clone(); // Clone the message to avoid borrowing issues

    // println!("{}", message); //Testing
    if let Ok(m) = BinanceFeedProcessor::process(&message) {
        HttpResponse::Ok().body(m)
    } else {
        HttpResponse::InternalServerError().body("Failed to retrieve feed")
    }
}
