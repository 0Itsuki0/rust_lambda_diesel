pub mod db_service;
pub mod handler;
pub mod model;
pub mod schema;
use axum::{routing::get, Router};
use bb8::Pool;
use lambda_http::run;
use lambda_http::{tracing, Error};
use std::env::set_var;

use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");
    let db_url = std::env::var("DATABASE_URL")?;
    let pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>> =
        get_connection_pool(&db_url).await?;

    let event_api = Router::new().route("/", get(handler::list_all_users));
    let app = Router::new().nest("/users", event_api).with_state(pool);

    run(app).await
}

pub async fn get_connection_pool(
    db_url: &str,
) -> Result<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>, Error> {
    // let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL environment variable");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = Pool::builder().build(config).await?;
    Ok(pool)
}
