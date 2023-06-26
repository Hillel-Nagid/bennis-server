use diesel::{
    Queryable,
    Insertable,
    deserialize::{ FromSql, FromSqlRow },
    pg::{ Pg, sql_types },
    sql_types::{ Text, Integer },
    serialize::{ ToSql, Output, self },
    expression::AsExpression,
};
use serde::{ Serialize, Deserialize };
use crate::schema::{ *, self };

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Integer)]
pub enum OrderStatus {
    Finished,
    Processing,
}
impl FromSql<Integer, Pg> for OrderStatus {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>
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

#[derive(Serialize, Deserialize, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = menu_items)]
pub struct MenuItem {
    pub id: i32,
    pub name: String,
    pub additions: Option<String>,
    pub price: f64,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = menu_items)]
pub struct NewMenuItem<'a> {
    pub name: &'a str,
    pub additions: Option<&'a str>,
    pub price: f64,
    pub image_url: Option<&'a str>,
}

#[derive(
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    AsChangeset,
    Identifiable,
    PartialEq,
    Clone
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

#[derive(Serialize, Deserialize, Debug, Insertable, PartialEq, Clone)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub customer_name: String,
    pub components: String,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Queryable, Selectable, AsChangeset, Eq, PartialEq)]
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
