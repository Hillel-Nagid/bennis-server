use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use diesel::{insert_into, prelude::*};

use crate::{
    models::*,
    schema::{menu_items, orders},
};
use dotenvy::dotenv;
use std::env;

fn establish_connection() -> PgConnection {
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

pub async fn post_order(Json(order): Json<NewOrder>) -> (StatusCode, Json<NewOrder>) {
    use crate::schema::customer_info::dsl::*;
    use crate::schema::orders::dsl::*;

    let connection = &mut establish_connection();
    if let Ok(users) = customer_info
        .filter(name.eq(&order.customer_name))
        .load::<CustomerInfo>(connection) as Result<Vec<CustomerInfo>, _>
    {
        if (&users).len() > 0 {
            let insertable_order = InsertableOrder {
                customer_id: users[0].clone().id,
                customer_name: users[0].clone().name,
                components: order.clone().components,
                price: order.clone().price,
            };
            let insertion = insert_into(orders)
                .values(&insertable_order)
                .execute(connection);
            assert_eq!(insertion, Ok(1));
        } else {
            let new_user = insert_into(customer_info)
                .values(NewCustomer {
                    name: order.clone().customer_name,
                    phone: Some("0525486556".into()),
                })
                .get_result::<CustomerInfo>(connection)
                .expect("Failed adding new user");
            let insertable_order = InsertableOrder {
                customer_id: new_user.id,
                customer_name: new_user.name,
                components: order.clone().components,
                price: order.clone().price,
            };
            let insertion = insert_into(orders)
                .values(&insertable_order)
                .execute(connection);
            assert_eq!(insertion, Ok(1));
        }
    }
    println!("{}", order);
    (StatusCode::CREATED, Json(order))
}

pub async fn get_single_order(Path(id): Path<i32>) -> (StatusCode, Json<Order>) {
    use crate::schema::orders::dsl::*;

    let connection = &mut establish_connection();
    let result = orders
        .find(id)
        .load::<Order>(connection)
        .expect("failed getting order");
    (StatusCode::OK, Json(result[0].clone()))
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
