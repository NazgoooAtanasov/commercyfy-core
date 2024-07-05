use commercyfy_core::{commercyfy_success, route_utils::CommercyfyResponse};

#[derive(serde::Serialize)]
pub struct Account {}

pub async fn get_account() -> CommercyfyResponse<Account> {
    return commercyfy_success!(Account{});
}

pub async fn create_account() -> CommercyfyResponse<Account> {
    return commercyfy_success!(Account{});
}

#[derive(serde::Serialize)]
pub struct Signin {}

pub async fn signin() -> CommercyfyResponse<Signin> {
    return commercyfy_success!(Signin{});
}
