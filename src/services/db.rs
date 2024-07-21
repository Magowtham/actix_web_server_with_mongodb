
use std::str::FromStr;
use std::time::SystemTime;

use chrono::Utc;
use futures_util::StreamExt;
use mongodb::bson::from_document;
use mongodb::bson::{doc,oid::ObjectId,DateTime};
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{Client,Collection,error::Error as MongoError};

use crate::models::booking_model::{Booking,FullBooking};
use crate::models::owner_model::Owner;
use crate::models::dog_model::Dog;



#[derive(Clone)]
pub struct Database {
    booking:Collection<Booking>,
    owner:Collection<Owner>,
    dog:Collection<Dog>,
}

impl Database {

    pub async fn init()-> Result<Self,MongoError>{
        let uri=std::env::var("DB_URI").expect("failed to read the env variable");

        let client=Client::with_uri_str(uri).await?;
        let db=client.database("dog_walkoing");

        let booking:Collection<Booking>=db.collection("booking");
        let owner:Collection<Owner>=db.collection("owner");
        let dog:Collection<Dog>=db.collection("dog");

        println!("connected to database");

        return Ok(Self {
            booking,
            owner,
            dog
        });
    }


    pub async fn create_owner(&self,owner:Owner)->Result<InsertOneResult,MongoError> {
        let result=self.owner.insert_one(owner).await?;
        return Ok(result);
    }

    pub async fn create_dog(&self,dog:Dog)->Result<InsertOneResult,MongoError> {
        let result=self.dog.insert_one(dog).await?;
        return Ok(result);
    }

    pub async fn create_booking(&self,booking:Booking) -> Result<InsertOneResult,MongoError> {
        let result=self.booking.insert_one(booking).await?;
        return Ok(result);
    }

    pub async fn cancel_booking(&self,booking_id:&str)->Result<UpdateResult,MongoError> {

        let booking_id=ObjectId::from_str(booking_id).expect("failed to parse to object id");

        let find_booking_query=doc! {
            "_id":booking_id
        };

        let booking_update_query=doc! {
            "$set": {
                "cancelled":true
            }
        };

        let result=self.booking.update_one(find_booking_query, booking_update_query).await?;

        return Ok(result);

}

    pub async fn get_bookings(&self) -> Result<Vec<FullBooking>,MongoError> {
        let now:SystemTime=Utc::now().into();

        let pipeline=vec![
            doc! {
                "$match":{
                    "cancelled":false,
                    "start_time":{
                        "$gte":DateTime::from(now)
                    }
                }
            },

            doc! {
                "$lookup": doc! {
                    "from":"owner",
                    "localField":"owner",
                    "foreignField":"_id",
                    "as":"owner"
                }
            },
            doc! {
                "$unwind" : doc! {
                    "path":"$owner"
                }
            },
            doc! {
                "$lookup":doc! {
                    "from":"owner",
                    "localField":"owner",
                    "foreignField":"_id",
                    "as":"owner"
                }
            }
        ];
    
        
        let mut result=self.booking.aggregate(pipeline).await?;

        let mut bookings:Vec<FullBooking>=Vec::new();

        while let Some(res)=result.next().await {
            match res {
                Ok(document) => {
                    let booking:FullBooking=from_document(document).expect("Error converting document to full booking");
                    bookings.push(booking);
                },
                Err(err) => panic!("Error getting booking: {}",err)
            }
        }

        return Ok(bookings);
    } 

}