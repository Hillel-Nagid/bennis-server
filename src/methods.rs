
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::{models::*, schema::{menu_items, orders}};


fn establish_connection() -> PgConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn get_orders() -> (StatusCode, Json<Vec<Order>>) {
    use crate::schema::orders::dsl::*;

    let connection = &mut establish_connection();
    let result = orders.load::<Order>(connection).expect("error");
    (StatusCode::OK, Json(result))
}

pub async fn post_order() -> &'static str {
    "hey"
}

pub async fn get_single_order() -> &'static str {
    "hey"
}

pub async fn update_order() -> &'static str {
    "hey"
}

pub async fn delete_order() -> &'static str {
    "hey"
}

pub async fn save_user_info() -> &'static str {
    "hey"
}

pub async fn get_menu_item() -> &'static str {
    "hey"
}

pub async fn post_menu_item() -> &'static str {
    "hey"
}

pub async fn update_menu_item() -> &'static str {
    "hey"
}

pub async fn delete_menu_item() -> &'static str {
    "hey"
}