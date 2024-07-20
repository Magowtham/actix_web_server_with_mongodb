mod models;
mod services;

use actix_web::{get,App,HttpServer,HttpResponse,Responder};

#[get("/")]
async fn handler() -> impl Responder {
    HttpResponse::Ok().body("Hello This is Gowtham MA Rust Developer at Google")
}


#[actix_web::main]
async fn main()->std::io::Result<()>{



    HttpServer::new(|| App::new().service(handler))
        .bind(("127.0.0.1",5000))?
        .run()
        .await



}