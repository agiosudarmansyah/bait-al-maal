use crate::AccountType;

#[tauri::command]
pub fn vue() {
    println!("I was invoked by the Invoker through Vue!")
}

#[tauri::command]
pub async fn create_account(
    name: String,
    icon_key: String,
    account_type: AccountType,
) {
    println!("Bruh")
}