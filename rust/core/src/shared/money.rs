use serde::{Serialize, Deserialize};

use crate::shared::sql_enum::SqlEnum;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub amount: i64,
    pub currency: Currency,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Currency {
    IDR,
    USA,
}

impl Money {
    pub fn new(amount: i64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn zero(currency: Currency) -> Self {
        Self { 
            amount: 0, 
            currency, 
        }
    }

    pub fn format(&self) -> &'static str {
        match self.currency {
            Currency::IDR => "Rp",
            Currency::USA => "$",
        }
    }
}

impl SqlEnum for Currency {
    fn to_sql(&self) -> &'static str {
        match self {
            Currency::IDR => "IDR",
            Currency::USA => "USA",
        }
    }

    fn from_sql(s: &str) -> Option<Self> {
        match s {
            "IDR" => Some(Currency::IDR),
            "USA" => Some(Currency::USA),
            _ => None,
        }
    }
}