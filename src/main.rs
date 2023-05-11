#![allow(dead_code)]
#![allow(unused_imports)]
use axum::{ routing::{ get, post }, http::StatusCode, response::IntoResponse, Json, Router };
use serde::{ Deserialize, Serialize };
use std::net::SocketAddr;
use mongodb::{
    Client,
    options::{ ClientOptions, ServerApi, ServerApiVersion },
    bson::doc,
    bson::oid::ObjectId,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let mut client_options = ClientOptions::parse("12123").await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;
    client.database("admin").run_command(doc! { "ping": 1 }, None).await?;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .route("/orders", get(get_orders))
        .route("/orders", post(post_order))
        .route("/order/:id", get(get_single_order))
        .route("/order/:id", post(update_order))
        .route("/saveInfo", get(save_user_info))
        .route("/menu", get(get_menu_item))
        .route("/menu", post(post_menu_item))
        .route("/menu/:id", post(update_menu_item));
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
    Ok(())
}

#[derive(Serialize, Deserialize)]
enum OrderStatus {
    Finished,
    Processing,
}

#[derive(Serialize, Deserialize)]
struct Addition {
    id: ObjectId,
    name: String,
    price: f64,
    image: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct MenuItem {
    id: ObjectId,
    name: String,
    additions: Vec<Addition>,
    price: f64,
    image: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct Order {
    id: ObjectId,
    name: String,
    ingridients: Vec<MenuItem>,
    status: OrderStatus,
    price: f64,
}

async fn get_orders() -> &'static str {
    "hey"
}

async fn post_order() {}

async fn get_single_order() {}

async fn update_order() {} //Delete OR Put

async fn save_user_info() {}

async fn get_menu_item() {}

async fn post_menu_item() {}

async fn update_menu_item() {} //Delete OR Put