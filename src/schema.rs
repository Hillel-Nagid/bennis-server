// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "order_status"))]
    pub struct OrderStatus;
}

diesel::table! {
    additions (id) {
        id -> Int4,
        name -> Text,
        price -> Float8,
        image_url -> Nullable<Text>,
    }
}

diesel::table! {
    customer_info (id) {
        id -> Int4,
        phone -> Nullable<Text>,
        name -> Text,
    }
}

diesel::table! {
    menu_items (id) {
        id -> Int4,
        name -> Text,
        additions -> Nullable<Text>,
        price -> Float8,
        image_url -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OrderStatus;

    orders (id) {
        id -> Int4,
        customer -> Nullable<Int4>,
        components -> Text,
        price -> Float8,
        status -> OrderStatus,
    }
}

diesel::joinable!(orders -> customer_info (customer));

diesel::allow_tables_to_appear_in_same_query!(
    additions,
    customer_info,
    menu_items,
    orders,
);
