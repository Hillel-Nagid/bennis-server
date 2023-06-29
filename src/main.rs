#![allow(unused)]
#[macro_use]
extern crate diesel;
mod methods;
mod models;
mod schema;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 1234));
    let app = Router::new()
        .route(
            "/orders",
            get(methods::get_orders).post(methods::post_order),
        )
        .route(
            "/order/:id",
            get(methods::get_single_order)
                .put(methods::update_order)
                .delete(methods::delete_order),
        )
        .route(
            "/menu",
            get(methods::get_menu).post(methods::post_menu_item),
        )
        .route(
            "/menu/:id",
            get(methods::get_menu_item)
                .put(methods::update_menu_item)
                .delete(methods::delete_menu_item),
        );
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
