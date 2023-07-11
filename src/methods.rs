use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use diesel::{delete, insert_into, prelude::*, update};

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

pub async fn get_orders() -> (StatusCode, Json<Vec<ParsableOrder>>) {
    use crate::schema::orders::dsl::*;

    let connection = &mut establish_connection();
    let result = orders
        .load::<Order>(connection)
        .expect("Failed getting orders");
    (
        StatusCode::OK,
        Json(result.iter().map(Order::parse_components).collect()),
    )
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

pub async fn get_single_order(Path(uid): Path<i32>) -> (StatusCode, Json<ParsableOrder>) {
    use crate::schema::orders::dsl::*;

    let connection = &mut establish_connection();
    let result = orders
        .find(uid)
        .load::<Order>(connection)
        .expect("failed getting order");
    (StatusCode::OK, Json(result[0].clone().parse_components()))
}

pub async fn update_order(
    Path(uid): Path<i32>,
    Json(new_status): Json<OrderStatus>,
) -> (StatusCode, Json<Order>) {
    use crate::schema::orders::dsl::*;

    let connection = &mut establish_connection();
    let updated_order = update(orders)
        .filter(id.eq(uid))
        .set(status.eq(new_status))
        .get_result::<Order>(connection)
        .expect("Failed updating order {}");
    (StatusCode::OK, Json(updated_order))
}

pub async fn delete_order(Path(id): Path<i32>) -> (StatusCode, Json<Order>) {
    use crate::schema::orders::dsl::*;

    let connection = &mut establish_connection();
    let deleted_order = delete(orders.filter(id.eq(id)))
        .get_result::<Order>(connection)
        .expect("Failed deleting order");
    (StatusCode::OK, Json(deleted_order))
}

pub async fn get_menu() -> (StatusCode, Json<Vec<ParsableMenuItem>>) {
    use crate::schema::menu_items::dsl::*;

    let connection = &mut establish_connection();
    let result = menu_items
        .load::<MenuItem>(connection)
        .expect("Failed getting the menu");

    (
        StatusCode::OK,
        Json(result.iter().map(MenuItem::parse_components).collect()),
    )
}

pub async fn get_menu_item(Path(uid): Path<i32>) -> (StatusCode, Json<ParsableMenuItem>) {
    use crate::schema::menu_items::dsl::*;

    let connection = &mut establish_connection();
    let result = menu_items
        .find(uid)
        .load::<MenuItem>(connection)
        .expect("failed getting menu item");
    (StatusCode::OK, Json(result[0].clone().parse_components()))
}

pub async fn post_menu_item(Json(item): Json<NewMenuItem>) -> (StatusCode, Json<NewMenuItem>) {
    use crate::schema::menu_items::dsl::*;

    let connection = &mut establish_connection();
    let insertion = insert_into(menu_items).values(&item).execute(connection);
    assert_eq!(insertion, Ok(1));
    println!("{}", item);
    (StatusCode::OK, Json(item))
}

pub async fn update_menu_item(
    Path(uid): Path<i32>,
    Json(updates): Json<UpdateableMenuItem>,
) -> (StatusCode, Json<MenuItem>) {
    use crate::schema::menu_items::dsl::*;

    let connection = &mut establish_connection();
    let updated_order: Result<Vec<MenuItem>, _> = menu_items.find(uid).load::<MenuItem>(connection);
    let mut item_for_update: MenuItem = match updated_order {
        Ok(items) => items[0].clone(),
        Err(e) => panic!("{e}"),
    };

    match updates.additions {
        Some(adds) => item_for_update.additions = Some(adds),
        None => (),
    };
    match updates.image_url {
        Some(url) => item_for_update.image_url = Some(url),
        None => (),
    };
    match updates.name {
        Some(n) => item_for_update.name = n,
        None => (),
    };
    match updates.price {
        Some(p) => item_for_update.price = p,
        None => (),
    };

    let updated_item = update(menu_items)
        .filter(id.eq(uid))
        .set(&item_for_update)
        .get_result(connection)
        .expect("Failed updating menu item");
    (StatusCode::OK, Json(updated_item))
}

pub async fn delete_menu_item(Path(uid): Path<i32>) -> (StatusCode, Json<MenuItem>) {
    use crate::schema::menu_items::dsl::*;

    let connection = &mut establish_connection();
    let deleted_item = delete(menu_items.filter(id.eq(uid)))
        .get_result::<MenuItem>(connection)
        .expect("Failed deleting menu item");
    (StatusCode::OK, Json(deleted_item))
}
