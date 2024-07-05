use commercyfy_core::{commercyfy_success, route_utils::CommercyfyResponse};

#[derive(serde::Serialize)]
pub struct ProductSearch {}

pub async fn product_search() -> CommercyfyResponse<ProductSearch> {
    return commercyfy_success!(ProductSearch{});
}
