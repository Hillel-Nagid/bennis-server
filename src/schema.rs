// @generated automatically by Diesel CLI.

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
        name -> Text,
        phone -> Nullable<Text>,
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
    orders (id) {
        id -> Int4,
        customer_id -> Int4,
        customer_name -> Text,
        components -> Text,
        price -> Float8,
        status -> Nullable<Int4>,
    }
}

diesel::joinable!(orders -> customer_info (customer_id));

diesel::allow_tables_to_appear_in_same_query!(additions, customer_info, menu_items, orders);
