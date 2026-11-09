use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct MenuItem {
    pub id: i32,
    pub name: String,
    pub price: Decimal,
    pub category: String,
    pub stock: i32,
}

#[derive(Debug, Deserialize)]
pub struct MenuInput {
    pub name: String,
    pub price: Decimal,
    pub category: String,
    #[serde(default)]
    pub stock: i32,
}

#[derive(Debug, Deserialize)]
pub struct MenuPatchInput {
    pub name: Option<String>,
    pub price: Option<Decimal>,
    pub category: Option<String>,
    pub stock: Option<i32>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Order {
    pub id: i32,
    pub customer_name: String,
    pub item_name: String,
    pub quantity: i32,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderInput {
    pub customer_name: String,
    pub item_name: String,
    pub quantity: i32,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderInput {
    pub customer_name: String,
    pub menu_id: i32,
    pub quantity: i32,
    pub payment_method: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderPatchInput {
    pub customer_name: Option<String>,
    pub item_name: Option<String>,
    pub quantity: Option<i32>,
    pub status: Option<String>,
}
