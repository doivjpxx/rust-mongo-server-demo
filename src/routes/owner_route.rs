use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

use crate::{
    models::owner_model::{Owner, OwnerRequest},
    services::db::Database,
};

#[post("/owner")]
pub async fn create_owner(db: Data<Database>, request: Json<OwnerRequest>) -> HttpResponse {
    match db
        .create_owner(
            Owner::try_from(OwnerRequest {
                name: request.name.clone(),
                email: request.email.clone(),
                phone: request.phone.clone(),
                addess: request.addess.clone(),
            })
            .expect("Error converting owner request to owner model"),
        )
        .await
    {
        Ok(owner) => HttpResponse::Created().json(owner),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
