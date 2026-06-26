/// Handlers and infrastructures (auth, database, sync) of the app.
pub mod app_state;
pub mod error;
pub mod infrastructure;

pub use crate::infrastructure::database::AppDatabase;

/// Features
pub mod account;
pub mod analytics;
pub mod budget;
pub mod categories;
pub mod user;

pub use crate::account::{
    Account,
    AccountType,
    AccountRepository
};
pub use crate::budget::{
    Budget,
};
pub use crate::categories::Categories;
pub use crate::user::UserCredentials;