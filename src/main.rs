mod models;
mod services;
mod routes;

use actix_web::{App,HttpServer};
use actix_web::web::Data;

use crate::routes::owner_route::create_owner;
use crate::routes::dog_route::create_dog;
use crate::routes::booking_route::{create_booking,cancel_booking,get_bookings};


#[actix_web::main]
async fn main()->std::io::Result<()>{

    let db=services::db::Database::init().await.expect("failed to connect to database");

    HttpServer::new(move || {
        let app_data=Data::new(db.clone());
        App::new().app_data(app_data)
        .service(create_owner)
        .service(create_dog)
        .service(create_booking)
        .service(get_bookings)
        .service(cancel_booking)
    })
        .bind(("127.0.0.1",5000))?
        .run()
        .await
}