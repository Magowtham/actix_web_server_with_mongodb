use mongodb::bson::{oid::ObjectId};
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Dog {
    pub _id:ObjectId,
    pub owner:ObjectId,
    pub name:Option<String>,
    pub age:Option<u8>,
    pub breed:Option<String>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct DogRequest  {
    pub name:String,
    pub owner:String,
    pub age:u8,
    pub breed:String
}

impl TryFrom<DogRequest> for Dog {
    type Error=Box<dyn std::error::Error>;

    fn try_from(item:DogRequest)->Result<Self,Self::Error> {
        return Ok(Self {
            _id:ObjectId::new(),
            owner:ObjectId::parse_str(&item.owner)?,
            name:Some(item.name),
            age:Some(item.age),
            breed:Some(item.breed)
        })
    }
} 