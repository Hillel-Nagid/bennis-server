#![allow(unused)]
#[macro_use]
extern crate diesel;
mod models;
mod schema;
mod methods;

use axum::{
    extract::State,
    routing::{ get, post, put, delete },
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
};
use serde::{ Deserialize, Serialize };
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 1234));
    let app = Router::new()
        .route("/orders", get(methods::get_orders).post(methods::post_order))
        .route(
            "/order/:id",
            get(methods::get_single_order).put(methods::update_order).delete(methods::delete_order)
        )
        .route("/saveInfo", post(methods::save_user_info))
        .route("/menu", get(methods::get_menu_item).post(methods::post_menu_item))
        .route("/menu/:id", put(methods::update_menu_item).delete(methods::delete_menu_item));
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
