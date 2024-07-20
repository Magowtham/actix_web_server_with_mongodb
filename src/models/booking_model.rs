use std::time::SystemTime;

use crate::models::owner_model::Owner;
use crate::models::dog_model::Dog;

use chrono::Utc;
use mongodb::bson::{oid::ObjectId,DateTime};
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Booking {
    pub _id:ObjectId,
    pub owner:ObjectId,
    pub start_time:DateTime,
    pub duration_in_minutes:u8,
    pub cancelled:bool
}

#[derive(Debug,Serialize,Deserialize)]
pub struct BookingRequest {
    pub owner:String,
    pub start_time:String,
    pub duration_in_minutes:u8
}

#[derive(Debug,Deserialize,Serialize)]
pub struct FullBooking {
    pub _id:ObjectId,
    pub owner:Owner,
    pub dogs:Vec<Dog>,
    pub start_time:DateTime,
    pub duration_in_minutes:u8,
    pub cancelled:bool
}

impl TryFrom<BookingRequest> for Booking {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item:BookingRequest)->Result<Self,Self::Error> {

        let chrono_datetime:SystemTime=chrono::DateTime::parse_from_rfc3339(&item.start_time)
            .map_err(|err| format!("Failed to parse start_time: {}",err))?
            .with_timezone(&Utc)
            .into();
        
        return Ok(Self {
            _id:ObjectId::new(),
            owner:ObjectId::parse_str(&item.owner)?,
            start_time:DateTime::from(chrono_datetime),
            duration_in_minutes:item.duration_in_minutes,
            cancelled:false
        });

    }
}