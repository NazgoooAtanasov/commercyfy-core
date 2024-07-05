use commercyfy_core::{commercyfy_success, route_utils::CommercyfyResponse};

#[derive(serde::Serialize)]
pub struct CategorySearch {}

pub async fn category_search() -> CommercyfyResponse<CategorySearch> {
    return commercyfy_success!(CategorySearch{});
}
