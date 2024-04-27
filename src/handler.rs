use axum::http::StatusCode;
use axum::{extract::State, response::Json};
use bb8::Pool;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use serde_json::{json, Value};

use crate::db_service;

pub async fn list_all_users(
    State(pool): State<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
) -> (StatusCode, Json<Value>) {
    let users = db_service::list_users(&pool).await;
    match users {
        Ok(users) => (
            StatusCode::OK,
            Json(json!({
                "error": true,
                "users": users
            })),
        ),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": true,
                "message": error.to_string()
            })),
        ),
    }
}
