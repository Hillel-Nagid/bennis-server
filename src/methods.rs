use axum::{ extract::{ State, Path }, http::StatusCode, Json };
use diesel::{ prelude::*, insert_into };

use dotenvy::dotenv;
use std::env;
use crate::{ models::*, schema::{ menu_items, orders } };

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_|
        panic!("Error connecting to {}", database_url)
    )
}

pub async fn get_orders() -> (StatusCode, Json<Vec<Order>>) {
    use crate::schema::orders::dsl::*;

    let connection = &mut establish_connection();
    let result = orders.load::<Order>(connection).expect("error");
    (StatusCode::OK, Json(result))
}

pub async fn post_order(Json(order): Json<NewOrder>) -> (StatusCode, Json<NewOrder>) {
    use crate::schema::orders::dsl::*;
    use crate::schema::customer_info::dsl::*;

    let connection = &mut establish_connection();
    if
        let Ok(user) = customer_info
            .filter(name.eq(&order.customer_name))
            .load::<CustomerInfo>(connection)
    {
        //returns an error that states that costumer_id is required.
        todo!("Make an insertable InsertableOrder struct");
        let insertion = insert_into(orders).values(&order).execute(connection);
        assert_eq!(insertion, Ok(1));
    } else {
        insert_into(customer_info)
            .values(NewCustomer {
                name: order.clone().customer_name,
                phone: Some("0525486556".into()),
            })
            .get_result::<CustomerInfo>(connection);
        let insertion = insert_into(orders).values(&order).execute(connection);
        assert_eq!(insertion, Ok(1));
    }
    (StatusCode::CREATED, Json(order))
}

pub async fn get_single_order(Path(id): Path<i32>) -> (StatusCode, Json<Order>) {
    use crate::schema::orders::dsl::*;

    let connection = &mut establish_connection();
    let result = orders.find(id).load::<Order>(connection).expect("failed getting order");
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
