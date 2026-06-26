use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::Categories;

#[derive(Debug, Serialize, Deserialize)]
pub struct Budget {
    pub id: Uuid,
    pub name: String,
    pub category: Categories,
    pub limit: u32,
}