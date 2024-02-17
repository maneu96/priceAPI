use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;
use tokio::sync::mpsc::Receiver;

pub async fn respond(receiver: web::Data<Mutex<Receiver<String>>>, ) -> impl Responder {
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

pub async fn echo(req_body: String) -> impl Responder {
    // Implementation here
    HttpResponse::Ok().body(req_body)
}