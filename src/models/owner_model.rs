use mongodb::bson::{oid::ObjectId};
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Owner {
    pub _id:ObjectId,
    pub name:String,
    pub email:String,
    pub phone:String,
    pub address:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct OwnerRequest {
    pub name:String,
    pub email:String,
    pub phone:String,
    pub address:String
}

impl TryFrom<OwnerRequest> for Owner {
    
    type Error=Box<dyn std::error::Error>;

    fn try_from(item:OwnerRequest) -> Result<Self,Self::Error> {
        return Ok(Self {
            _id:ObjectId::new(),
            name:item.name,
            email:item.email,
            phone:item.phone,
            address:item.address
        });
    }
}



