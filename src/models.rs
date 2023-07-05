use std::fmt::Display;

use crate::schema::{self, *};
use axum::Json;
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{sql_types, Pg},
    serialize::{self, Output, ToSql},
    sql_types::{Integer, Text},
    Insertable, Queryable,
};
use serde::{Deserialize, Serialize};
use serde_json;
trait Parsable {
    fn parse_components<T>(&self) -> Json<T>;
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Integer)]
pub enum OrderStatus {
    Finished,
    Processing,
}
impl FromSql<Integer, Pg> for OrderStatus {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match <i32 as FromSql<Integer, Pg>>::from_sql(bytes)? {
            0 => Ok(OrderStatus::Processing),
            1 => Ok(OrderStatus::Finished),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}
impl ToSql<Integer, Pg> for OrderStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            OrderStatus::Processing => <i32 as ToSql<Integer, Pg>>::to_sql(&0, out),
            OrderStatus::Finished => <i32 as ToSql<Integer, Pg>>::to_sql(&1, out),
        }
    }
}
#[derive(Serialize, Deserialize, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = additions)]
pub struct Addition {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = additions)]
pub struct NewAddition<'a> {
    pub name: &'a str,
    pub price: f64,
    pub image_url: Option<&'a str>,
}

pub struct ParsableMenuItem {
    pub id: i32,
    pub name: String,
    pub additions: Result<Vec<Addition>, serde_json::Error>,
    pub price: f64,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Queryable, Selectable, AsChangeset, Clone)]
#[diesel(table_name = menu_items)]
pub struct MenuItem {
    pub id: i32,
    pub name: String,
    pub additions: Option<String>,
    pub price: f64,
    pub image_url: Option<String>,
}
impl Parsable for MenuItem {
    fn parse_components(&self) -> Json<ParsableMenuItem> {
        let parsed: ParsableMenuItem;
        if let Some(additions) = self.additions {
            parsed = ParsableMenuItem {
                id: self.id,
                name: self.name,
                additions: serde_json::from_str(&additions),
                price: self.price,
                image_url: self.image_url,
            };
        } else {
            parsed = ParsableMenuItem {
                id: self.id,
                name: self.name,
                additions: Ok(vec![]),
                price: self.price,
                image_url: self.image_url,
            };
        }
        Json(parsed)
    }
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = menu_items)]
pub struct NewMenuItem {
    pub name: String,
    pub additions: Option<String>,
    pub price: f64,
    pub image_url: Option<String>,
}
impl Parsable for NewMenuItem {}
impl Display for NewMenuItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "A new dish ({}) was just added and its base price is {} shekels",
            self.name, self.price
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]

pub struct UpdateableMenuItem {
    pub name: Option<String>,
    pub additions: Option<String>,
    pub price: Option<f64>,
    pub image_url: Option<String>,
}

#[derive(
    Serialize, Deserialize, Queryable, Selectable, AsChangeset, Identifiable, PartialEq, Clone,
)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i32,
    pub customer_id: i32,
    pub customer_name: String,
    pub components: String,
    pub price: f64,
    pub status: Option<OrderStatus>,
}
impl Parsable for Order {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NewOrder {
    pub customer_name: String,
    pub components: String,
    pub price: f64,
}
impl Parsable for NewOrder {}

impl Display for NewOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} ordered {} for {} shekels",
            self.customer_name, self.components, self.price
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Insertable, PartialEq, Clone)]
#[diesel(table_name = orders)]
pub struct InsertableOrder {
    pub customer_id: i32,
    pub customer_name: String,
    pub components: String,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Queryable, Selectable, AsChangeset, Eq, PartialEq, Clone)]
#[diesel(table_name = customer_info)]
pub struct CustomerInfo {
    pub id: i32,
    pub name: String,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Insertable, Queryable)]
#[diesel(table_name = customer_info)]
pub struct NewCustomer {
    pub name: String,
    pub phone: Option<String>,
}
