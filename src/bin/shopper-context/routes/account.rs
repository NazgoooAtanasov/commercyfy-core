use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use commercyfy_core::{
    commercyfy_fail, commercyfy_success,
    route_utils::{extractors::StructuredJson, CommercyfyResponse, CreatedEntryResponse},
    schemas::account::CreateAccount,
    services::db::DbService,
    utils,
};

use super::ShopperContextState;

#[derive(serde::Serialize)]
pub struct Account {}
pub async fn get_account() -> CommercyfyResponse<Account> {
    return commercyfy_success!(Account {});
}

pub async fn create_account(
    State(state): State<Arc<ShopperContextState>>,
    StructuredJson(mut payload): StructuredJson<CreateAccount>,
) -> CommercyfyResponse<CreatedEntryResponse> {
    if let Ok(acc) = state.db_service.get_account_by_email(&payload.email).await {
        if let Some(_) = acc {
            return commercyfy_fail!("Account with that email already exists.".to_string());
        }
    }

    match utils::passwords::hash_password(&payload.password) {
        Ok(hash) => payload.password = hash,
        Err(err) => return commercyfy_fail!(err.to_string()),
    }

    return match state.db_service.create_account(&payload).await {
        Ok(accont) => {
            commercyfy_success!(StatusCode::CREATED, CreatedEntryResponse { id: accont.id })
        }
        Err(err) => commercyfy_fail!(err.to_string()),
    };
}

#[derive(serde::Serialize)]
pub struct Signin {}
pub async fn signin() -> CommercyfyResponse<Signin> {
    return commercyfy_success!(Signin {});
}
