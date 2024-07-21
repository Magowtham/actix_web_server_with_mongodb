use actix_web::{post,web::{Data,Json},HttpResponse};

use crate::{models::dog_model::{Dog,DogRequest},services::db::Database};

#[post("/dog")]
pub async fn create_dog(db:Data<Database>,request:Json<DogRequest>)->HttpResponse {

    let dog=Dog::try_from(DogRequest {
        owner:request.owner.clone(),
        name:request.name.clone(),
        age:request.age.clone(),
        breed:request.breed.clone()
    }).expect("Failed to parse the http request");

    let response=db.create_dog(dog).await;

    match response {
        Ok(dog) => HttpResponse::Ok().json(dog),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}