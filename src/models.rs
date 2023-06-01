use diesel::{Queryable, Insertable, deserialize::{FromSql, FromSqlRow}, pg::Pg, sql_types::Text, serialize::{ToSql, Output, self}, expression::AsExpression};
use serde::{Serialize, Deserialize};
use crate::schema::{*, self};

#[derive(Serialize, Deserialize, Debug,AsExpression)]
#[sql_type="schema::sql_types::OrderStatus"]
pub enum OrderStatus {
    Finished,
    Processing,
}


#[derive(Serialize, Deserialize,Queryable,Selectable,AsChangeset)]
#[table_name="additions"]
pub struct Addition {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "additions"]
pub struct NewAddition<'a>{
    pub name: &'a str,
    pub price: f64,
    pub image_url:Option<&'a str>
}

#[derive(Serialize, Deserialize,Queryable,Selectable,AsChangeset)]
#[table_name="menu_items"]
struct MenuItem {
    pub id: i32,
    pub name: String,
    pub additions: Option<String>,
    pub price: f64,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "menu_items"]
pub struct NewMenuItem<'a>{
    pub name: &'a str,
    pub additions: Option<&'a str>,
    pub price: f64,
    pub image_url:Option<&'a str>
}

#[derive(Serialize, Deserialize,Queryable,Selectable,AsChangeset)]
#[table_name="orders"]
pub struct Order {
    pub id: i32,
    pub customer: Option<i32>,
    pub components: String,
    pub status: OrderStatus,
    pub price: f64,
}


#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "orders"]
pub struct NewOrder<'a>{
    pub customer: Option<i32>,
    pub components: &'a str,
    pub status: OrderStatus,
    pub price: f64
}

#[derive(Serialize, Deserialize,Queryable,Selectable,AsChangeset)]
#[table_name="customer_info"]
pub struct CustomerInfo {
    pub id: i32,
    pub name: String,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "customer_info"]
pub struct NewCustomer<'a>{
    pub name: &'a str,
    pub phone: Option<&'a str>
}