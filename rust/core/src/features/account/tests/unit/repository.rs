use crate::account::{ Account, AccountType };
use crate::infrastructure::database::TursoAccountRepository;
use crate::shared::money::{ Money, Currency };

fn account_new() -> Account {
    return Account::new(
        String::from("Aigo Cash"),
        String::from("Banknote"),
        AccountType::Cash,
        Money::new(100_000, Currency::IDR),
    )
}