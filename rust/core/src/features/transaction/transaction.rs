use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::Categories;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction<T> {
    id: Uuid,
    name: String,
    amount: T,
    category: Categories,
}

impl<T> Transaction<T> {
    pub fn new (
        id: Uuid,
        name: String,
        amount: T,
        category: Categories,
    ) -> Self {
        Self {
            id,
            name,
            amount,
            category,
        }
    }
}