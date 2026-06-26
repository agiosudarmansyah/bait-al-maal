use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCredentials {
    id: Uuid,
    username: String,
    email: String,
    password: String,
    phone_number: i16,
}