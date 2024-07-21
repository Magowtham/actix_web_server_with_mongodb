use actix_web::{post,get,put,HttpResponse,web::{Data,Path,Json}};
use crate::{models::booking_model::{Booking,BookingRequest},services::db::Database};

#[post("/book")]
pub async fn create_booking(db:Data<Database>,request:Json<BookingRequest>)->HttpResponse{
    
    let booking=Booking::try_from(BookingRequest {
        owner:request.owner.clone(),
        start_time:request.start_time.clone(),
        duration_in_minutes:request.duration_in_minutes.clone()
    }).expect("failed to parse the http request");

    let response=db.create_booking(booking).await;

    match response {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[get("/bookings")]
pub async fn get_bookings(db:Data<Database>)->HttpResponse {
    match db.get_bookings().await {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(err)=>HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[put("/booking/{id}/cancel")]
pub async fn cancel_booking(db:Data<Database>,path:Path<String>) -> HttpResponse {

    let id=path.into_inner();

    match db.cancel_booking(id.as_str()).await {
        Ok(cancel_booking)=>HttpResponse::Ok().json(cancel_booking),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}