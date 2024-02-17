/* ************************************************************************************************************************************/
// This is an implementation of an API responder module
// It is called by the main, whenever a response to a client request is necessary.
// respond returns either a Responder with a string with an HTTP message or an error
/* ************************************************************************************************************************************/
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;
use tokio::sync::mpsc::Receiver;

pub async fn respond(receiver: web::Data<Mutex<Receiver<String>>>) -> impl Responder {
    /***********************************************************************************************************************/
    //
    //  inputs : ***********************************************************************************************************
    //          receiver: web::Data<Mutex<Receiver<String>> // A Mutex encapsulated by the Data structure of the API. 
    //                                                         it contains the receiver which can access the string 
    //                                                         of the message to be transmitted to the client
    // outputs: ************************************************************************************************************
    //           impl Responder // An implied Responder that contains an HTTP response or an error message
    /***********************************************************************************************************************/
    // Attempt to lock the receiver, handling the potential error without panicking
    let lock_result = receiver.lock();

    match lock_result {
        Ok(mut receiver) => {
            // Successfully acquired the lock, following step is to try receiving a message
            match receiver.try_recv() {
                Ok(message) => HttpResponse::Ok().body(message),
                Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve feed"),
            }
        }
        Err(e) => {
            // Handling the "mutex poisoning case", or other errors in acquiring the lock
            eprintln!("Failed to lock the receiver: {:?}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
