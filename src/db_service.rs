use crate::{model, schema};
use bb8::{Pool, PooledConnection};

use anyhow::{Ok, Result};
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use model::{
    events::{Event, NewEvent},
    users::{NewUser, User},
};

pub async fn list_users(
    pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
) -> Result<Vec<User>> {
    use schema::users::dsl::*;

    // get connection
    let mut conn: PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>> =
        pool.get().await?;

    let results = users.load::<User>(&mut conn).await?;
    Ok(results)
}

pub async fn get_user(
    pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    user_id: &str,
) -> Result<User> {
    use schema::users::dsl::*;

    // get connection
    let mut conn: PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>> =
        pool.get().await?;

    let user = users
        .find(user_id)
        .select(User::as_select())
        .first(&mut conn)
        .await?;

    Ok(user)
}

pub async fn get_event(
    pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    event_id: &str,
) -> Result<(Event, User)> {
    use schema::events::dsl::*;
    use schema::users::dsl::*;

    // get connection
    let mut conn: PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>> =
        pool.get().await?;

    let event = events
        .find(event_id)
        .inner_join(users)
        .select((Event::as_select(), User::as_select()))
        .first::<(Event, User)>(&mut conn)
        .await?;

    Ok(event)
}

pub async fn get_event_for_user(
    pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    userid: &str,
) -> Result<Vec<Event>> {
    use schema::events::dsl::*;
    // get connection
    let mut conn: PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>> =
        pool.get().await?;
    let events_for_user = events
        // .select(Event::as_select())
        // .filter(sql::<Bool>(" user_id = ").bind::<Text, _>(userid))
        .filter(user_id.eq(userid))
        .get_results(&mut conn)
        .await?;

    Ok(events_for_user)
}

pub async fn create_single_user(
    pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    new_user: NewUser,
) -> Result<User> {
    use schema::users;

    let mut conn: PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>> =
        pool.get().await?;

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await?;

    Ok(user)
}

pub async fn create_multiple_user(
    pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    new_users: Vec<NewUser>,
) -> Result<Vec<User>> {
    use schema::users;

    let mut conn: PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>> =
        pool.get().await?;

    let users = diesel::insert_into(users::table)
        .values(&new_users)
        .returning(User::as_returning())
        .get_results(&mut conn)
        .await?;

    Ok(users)
}

pub async fn update_username(
    pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    id: &str,
    new_username: &str,
) -> Result<User> {
    use schema::users::dsl::{username, users};

    let mut conn: PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>> =
        pool.get().await?;

    let user = diesel::update(users.find(id))
        .set(username.eq(new_username))
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await?;

    Ok(user)
}

pub async fn delete_user(
    pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    user_id: &str,
) -> Result<usize> {
    use schema::users::dsl::*;

    // get connection
    let mut conn: PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>> =
        pool.get().await?;

    let num_deleted = diesel::delete(users.find(user_id))
        .execute(&mut conn)
        .await?;

    Ok(num_deleted)
}

pub async fn create_single_event(
    pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    new_event: NewEvent,
) -> Result<Event> {
    use schema::events;

    let mut conn: PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>> =
        pool.get().await?;

    let event = diesel::insert_into(events::table)
        .values(&new_event)
        .returning(Event::as_returning())
        .get_result(&mut conn)
        .await?;

    Ok(event)
}
