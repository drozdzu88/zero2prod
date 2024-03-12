use actix_web::{web, App, HttpResponse};
use std::net::TcpListener; //we will bind the port on our own with TcpListener and then hand
                           // that over to the HttpServer using listen.


#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}