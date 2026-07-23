use crate::account::{Account, AccountService, AccountType, MockAccountRepository, ProviderBank};
use crate::shared::{
    error::Result,
    money::{Currency, Money},
};

#[tokio::test]
async fn service_creates_account() -> Result<()> {
    let mut mock = MockAccountRepository::new();

    mock.expect_create()
        .withf(|account| {
            account.name == "Agio"
                && account.icon_key == "Bank"
                && account.account_type == AccountType::Bank(ProviderBank::BCA)
                && account.balance == Money::new(10_000, Currency::IDR)
        })
        .times(1)
        .returning(move |_| Ok(()));

    let service = AccountService::new(mock);

    let actual = service
        .create(
            String::from("Agio"),
            String::from("Bank"),
            AccountType::Bank(ProviderBank::BCA),
            Money::new(10_000, Currency::IDR),
        )
        .await;

    assert!(actual.is_ok());

    Ok(())
}
