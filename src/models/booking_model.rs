use std::convert::TryFrom;
use std::time::SystemTime;

use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use super::dog_model::Dog;

#[derive(Debug, Serialize, Deserialize)]
pub struct Booking {
    pub _id: ObjectId,
    pub owner: ObjectId,
    pub start_time: DateTime,
    pub duration_in_minutes: u8,
    pub cancelled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingRequest {
    pub owner: String,
    pub start_time: String,
    pub duration_in_minutes: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullBooking {
    pub _id: String,
    pub owner: String,
    pub dogs: Vec<Dog>,
    pub start_time: String,
    pub duration_in_minutes: u8,
    pub cancelled: bool,
}

impl TryFrom<BookingRequest> for Booking {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: BookingRequest) -> Result<Self, Self::Error> {
        let chrono_datetime: SystemTime = chrono::DateTime::parse_from_rfc3339(&value.start_time)?
            .with_timezone(&chrono::Utc)
            .into();

        Ok(Self {
            _id: ObjectId::new(),
            owner: ObjectId::parse_str(&value.owner)?,
            start_time: DateTime::from(chrono_datetime),
            duration_in_minutes: value.duration_in_minutes,
            cancelled: false,
        })
    }
}
