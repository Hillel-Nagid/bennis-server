#![allow(unused)]
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
use mongodb::{
    Client,
    options::{ ClientOptions, ServerApi, ServerApiVersion, FindOptions },
    bson::doc,
    bson::oid::ObjectId,
};
use futures::stream::TryStreamExt;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017/").await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;
    client.database("admin").run_command(doc! { "ping": 1 }, None).await?;
    let db: mongodb::Database = client.database("bennis");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .route("/orders", get(get_orders).post(post_order))
        .route("/order/:id", get(get_single_order).put(update_order).delete(delete_order))
        .route("/saveInfo", post(save_user_info))
        .route("/menu", get(get_menu_item).post(post_menu_item))
        .route("/menu/:id", put(update_menu_item).delete(delete_menu_item))
        .with_state(db);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
enum OrderStatus {
    Finished,
    Processing,
}

#[derive(Serialize, Deserialize, Debug)]
struct Addition {
    id: ObjectId,
    name: String,
    price: f64,
    image: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MenuItem {
    id: ObjectId,
    name: String,
    additions: Vec<Addition>,
    price: f64,
    image: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    id: ObjectId,
    name: String,
    ingridients: Vec<MenuItem>,
    status: OrderStatus,
    price: f64,
}

#[derive(Serialize, Deserialize)]
struct UserInfo {
    id: ObjectId,
    name: String,
    phone: String,
}

async fn get_orders(State(db): State<mongodb::Database>) -> (StatusCode, Json<Vec<Order>>) {
    let collection: mongodb::Collection<Order> = db.collection::<Order>("orders");
    let mut orders = collection
        .find(doc! {}, FindOptions::builder().build()).await
        .ok()
        .expect("Failed getting orders");
    // let ser_orders: Vec<Order> = orders.try_collect().await.ok().expect("Failed serializing");
    let mut final_orders: Vec<Order> = vec![];
    while let Some(order) = orders.try_next().await.expect("Failed method 2") {
        final_orders.push(order);
    }
    println!("{:?}", final_orders);
    // for order in ser_orders {
    //     final_orders.push(order);
    // }
    (StatusCode::OK, Json(final_orders))
}

async fn post_order(State(db): State<mongodb::Database>) -> &'static str {
    let collection: mongodb::Collection<Order> = db.collection::<Order>("orders");
    collection;
    "hey"
}

async fn get_single_order(State(db): State<mongodb::Database>) -> &'static str {
    let collection: mongodb::Collection<Order> = db.collection::<Order>("orders");
    collection;
    "hey"
}

async fn update_order(State(db): State<mongodb::Database>) -> &'static str {
    let collection: mongodb::Collection<Order> = db.collection::<Order>("orders");
    collection;
    "hey"
}

async fn delete_order(State(db): State<mongodb::Database>) -> &'static str {
    let collection: mongodb::Collection<Order> = db.collection::<Order>("orders");
    "hey"
}

async fn save_user_info(State(db): State<mongodb::Database>) -> &'static str {
    let collection: mongodb::Collection<UserInfo> = db.collection::<UserInfo>("users");
    "hey"
}

async fn get_menu_item(State(db): State<mongodb::Database>) -> &'static str {
    let collection: mongodb::Collection<MenuItem> = db.collection::<MenuItem>("menu");
    "hey"
}

async fn post_menu_item(State(db): State<mongodb::Database>) -> &'static str {
    let collection: mongodb::Collection<MenuItem> = db.collection::<MenuItem>("menu");
    "hey"
}

async fn update_menu_item(State(db): State<mongodb::Database>) -> &'static str {
    let collection: mongodb::Collection<MenuItem> = db.collection::<MenuItem>("menu");
    "hey"
}

async fn delete_menu_item(State(db): State<mongodb::Database>) -> &'static str {
    let collection: mongodb::Collection<MenuItem> = db.collection::<MenuItem>("menu");
    "hey"
}