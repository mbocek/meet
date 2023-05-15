use crate::AppState;
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, HttpResponse, Responder};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::user::user_entity as User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDTO {
    id: Option<i32>,
    name: String,
}

impl UserDTO {
    pub fn into_active_model(self) -> User::ActiveModel {
        User::ActiveModel {
            name: Set(self.name.to_owned()),
            ..Default::default()
        }
    }
}

#[get("/")]
pub async fn users(data: web::Data<AppState>) -> impl Responder {
    let users_result = User::Entity::find().all(&data.conn).await;
    match users_result {
        Ok(users) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/")]
pub async fn add_users(data: web::Data<AppState>, user: web::Json<UserDTO>) -> impl Responder {
    user.clone()
        .into_active_model()
        .insert(&data.conn)
        .await
        .unwrap();
    HttpResponse::Ok()
}
