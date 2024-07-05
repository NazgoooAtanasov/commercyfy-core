use commercyfy_core::{commercyfy_success, route_utils::CommercyfyResponse};

#[derive(serde::Serialize)]
pub struct Basket {}

pub async fn get_basket() -> CommercyfyResponse<Basket> {
    return commercyfy_success!(Basket{});
}
