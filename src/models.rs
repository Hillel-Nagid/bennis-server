use std::fmt::Display;

use crate::schema::{
    self,
    orders::{components, customer_id},
    *,
};
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
#[derive(Serialize, Deserialize, Queryable, Selectable, AsChangeset, Debug, PartialEq, Clone)]
#[diesel(table_name = additions)]
pub struct Addition {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ParsableMenuItem {
    pub id: i32,
    pub name: String,
    pub additions: Vec<Addition>,
    pub price: f64,
    pub image_url: String,
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
impl MenuItem {
    pub fn parse_components(&self) -> ParsableMenuItem {
        let parsed: ParsableMenuItem;
        if let Some(additions) = self.clone().additions {
            parsed = ParsableMenuItem {
                id: self.id,
                name: self.clone().name,
                additions: serde_json::from_str(&additions).expect("failed parsing additions"),
                price: self.price,
                image_url: self.clone().image_url.unwrap_or(String::from("")),
            };
        } else {
            parsed = ParsableMenuItem {
                id: self.id,
                name: self.clone().name,
                additions: vec![],
                price: self.price,
                image_url: self.clone().image_url.unwrap_or(String::from("")),
            };
        }
        parsed
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
impl NewMenuItem {
    pub fn parse_components(&self, id: i32) -> ParsableMenuItem {
        let parsed: ParsableMenuItem;
        if let Some(additions) = self.additions.clone() {
            parsed = ParsableMenuItem {
                id,
                name: (*self.name).to_owned(),
                additions: serde_json::from_str(&additions).expect("failed parsing additions"),
                price: self.price,
                image_url: self.image_url.clone().unwrap_or(String::from("")),
            };
        } else {
            parsed = ParsableMenuItem {
                id: id,
                name: self.name.clone(),
                additions: vec![],
                price: self.price,
                image_url: self.image_url.clone().unwrap_or(String::from("")),
            };
        }
        parsed
    }
}
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
#[derive(Serialize, Deserialize, Debug)]
pub struct ParsableOrder {
    pub id: i32,
    pub customer_id: i32,
    pub customer_name: String,
    pub components: Vec<ParsableMenuItem>,
    pub price: f64,
    pub status: OrderStatus,
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

impl Order {
    pub fn parse_components(&self) -> ParsableOrder {
        let parsed: ParsableOrder;
        parsed = ParsableOrder {
            id: self.id,
            customer_id: self.customer_id,
            customer_name: self.customer_name.clone(),
            components: serde_json::from_str(&self.components).expect("failed parsing components"),
            price: self.price,
            status: self.status.unwrap_or(OrderStatus::Processing),
        };
        parsed
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NewOrder {
    pub customer_name: String,
    pub components: String,
    pub price: f64,
}

impl NewOrder {
    pub fn parse_components(&self, id: i32, cust_id: i32) -> ParsableOrder {
        let parsed: ParsableOrder;
        parsed = ParsableOrder {
            id,
            customer_id: cust_id,
            customer_name: self.customer_name.clone(),
            components: serde_json::from_str(&self.components).expect("failed parsing components"),
            price: self.price,
            status: OrderStatus::Processing,
        };
        parsed
    }
}

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
