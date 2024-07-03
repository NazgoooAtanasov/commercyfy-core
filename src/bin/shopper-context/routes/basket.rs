use commercyfy_core::{commercyfy_success, routes::CommercyfyResponse};

#[derive(serde::Serialize)]
pub struct Basket {}

pub async fn get_basket() -> CommercyfyResponse<Basket> {
    return (axum::http::StatusCode::OK,axum::Json(commercyfy_core::routes::CommercyfyResponseData::Success(Basket{})));
}
