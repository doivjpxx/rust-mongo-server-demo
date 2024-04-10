use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub _id: ObjectId,
    pub name: String,
    pub email: Option<String>,
    pub phone: String,
    pub addess: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone: String,
    pub addess: String,
}

impl TryFrom<OwnerRequest> for Owner {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: OwnerRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            name: value.name,
            email: value.email,
            phone: value.phone,
            addess: value.addess,
        })
    }
    
}