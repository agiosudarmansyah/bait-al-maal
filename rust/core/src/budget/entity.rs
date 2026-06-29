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

impl Budget {
    pub fn new(
        name: String,
        category: Categories,
        limit: u32,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
            category,
            limit,
        }
    }
}