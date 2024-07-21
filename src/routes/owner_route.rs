use actix_web::{post,web::{Data,Json},HttpResponse};
use crate::{models::owner_model::{Owner,OwnerRequest},services::db::Database};

#[post("/owner")]
pub async fn create_owner(db:Data<Database>,request:Json<OwnerRequest>) -> HttpResponse {
    let owner=Owner::try_from(OwnerRequest {
        name:request.name.clone(),
        email:request.email.clone(),
        phone:request.phone.clone(),
        address:request.address.clone()
    }).expect("failed to parse the http request");

    let response=db.create_owner(owner).await;

    match response {
        Ok(owner)=> HttpResponse::Ok().json(owner),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}