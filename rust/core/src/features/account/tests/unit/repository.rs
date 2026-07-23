use uuid::Uuid;

use crate::account::{ AccountRepository };
use crate::account::tests::unit::helper::*;
use crate::shared::error::{ Error, Result };

// create()

#[tokio::test]
async fn create_account_persist_account() -> Result<()> {
    // Arrange
    let ctx = setup().await;
    
    let account = new_account();

    // Act
    ctx.repo.create(&account).await?;

    // Assert
    let saved = ctx
        .repo
        .require_by_id(account.id)
        .await?;

    assert_eq!(saved.id, account.id);
    assert_eq!(saved.name, account.name);
    assert_eq!(saved.icon_key, account.icon_key);
    assert_eq!(saved.account_type, account.account_type);
    assert_eq!(saved.balance, account.balance);

    Ok(())
}

// find_by_id()

#[tokio::test]
async fn find_by_id_returns_account_when_exists() -> Result<()> {
    let ctx = setup().await;
    
    let account = new_account();

    ctx.repo.create(&account).await?;

    let saved = ctx
        .repo
        .find_by_id(account.id)
        .await?;

    assert_eq!(saved, Some(account));

    Ok(())
}

#[tokio::test]
async fn find_by_id_returns_none_when_missing() -> Result<()> {
    let ctx = setup().await;

    let id = Uuid::now_v7();

    let saved = ctx
        .repo
        .find_by_id(id)
        .await?;

    assert_eq!(saved, None);

    Ok(())
}

#[tokio::test]
async fn require_by_id_returns_account_when_exists() -> Result<()> {
    let ctx = setup().await;
    
    let account = new_account();

    ctx.repo.create(&account).await?;

    let saved = ctx
        .repo
        .require_by_id(account.id)
        .await?;

    assert_eq!(saved, account);

    Ok(())
}

#[tokio::test]
async fn require_by_id_returns_not_found_when_missing() -> Result<()> {
    let ctx = setup().await;

    let id = Uuid::now_v7();

    let result = ctx
        .repo
        .require_by_id(id)
        .await;

    assert!(matches!(result, Err(Error::NotFound)));

    Ok(())
}

#[tokio::test]
async fn require_all_returns_empty_when_database_is_empty() -> Result<()> {
    let ctx = setup().await;

    let saved = ctx
        .repo
        .require_all()
        .await?;

    assert_eq!(saved, vec![]);

    Ok(())
}

#[tokio::test]
async fn require_all_returns_all_accounts() -> Result<()> {
    let ctx = setup().await;

    let mut accounts = Vec::new();

    let mut number = 3;

    while number != 0 {
        let account = new_account();

        ctx.repo.create(&account).await?;

        accounts.push(account);

        number -= 1
    }

    let saved = ctx
        .repo
        .require_all()
        .await?;

    assert_eq!(saved, accounts);
    
    Ok(())
}

#[tokio::test]
async fn delete_removes_account() -> Result<()> {
    let ctx = setup().await;

    let account = new_account();

    ctx.repo.create(&account).await?;
    ctx.repo.delete(account.id).await?;

    let result = ctx
        .repo
        .find_by_id(account.id)
        .await?;

    assert_eq!(result, None);
    
    Ok(())
}

#[tokio::test]
async fn delete_missing_account_returns_not_found() -> Result<()> {
    let ctx = setup().await;

    let id = Uuid::now_v7();

    let result = ctx.repo.delete(id).await;

    assert!(matches!(result, Err(Error::NotFound)));

    Ok(())
}