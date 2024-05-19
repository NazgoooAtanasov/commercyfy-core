use super::{CommercyfyResponse, CreatedEntryResponse};
use crate::{
    models::{category::Category, product::Product},
    schemas::category::CreateCategory,
    services::db::DbService,
    CommercyfyExtrState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn get_categories(
    State(state): CommercyfyExtrState,
) -> CommercyfyResponse<Vec<Category>> {
    let categories = state.db_service.get_categories().await;

    if let Err(error) = categories {
        return commercyfy_fail!(error.to_string());
    }

    return commercyfy_success!(categories.unwrap());
}

pub async fn create_category(
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreateCategory>,
) -> CommercyfyResponse<CreatedEntryResponse> {
    if let Err(error) = payload.validate() {
        return commercyfy_fail!(error);
    }

    if let Ok(existing) = state
        .db_service
        .get_category_by_reference(&payload.category_reference)
        .await
    {
        if let Some(_) = existing {
            return commercyfy_fail!(format!(
                "Category with 'category_reference' '{}' already exists",
                payload.category_reference
            ));
        }
    }

    let created = state.db_service.create_category(payload).await;
    if let Err(error) = created {
        return commercyfy_fail!(error.to_string());
    }

    return commercyfy_success!(
        StatusCode::CREATED,
        CreatedEntryResponse {
            id: created.unwrap().id
        }
    );
}

#[derive(serde::Serialize)]
pub struct CategoryView {
    #[serde(flatten)]
    category: Category,
    products: Vec<Product>,
}
pub async fn get_category(
    State(state): CommercyfyExtrState,
    Path(id): Path<String>,
) -> CommercyfyResponse<CategoryView> {
    if let Ok(category) = state.db_service.get_category_by_id(&id).await {
        if let Some(cat) = category {
            let mut category_view = CategoryView {
                category: cat,
                products: Vec::new(),
            };

            if let Ok(products) = state.db_service.get_category_products_by_id(&id).await {
                category_view.products = products;
            }

            return commercyfy_success!(category_view);
        }
    }

    if let Ok(category) = state.db_service.get_category_by_reference(&id).await {
        if let Some(cat) = category {
            let mut category_view = CategoryView {
                category: cat,
                products: Vec::new(),
            };

            if let Ok(products) = state.db_service.get_category_products_by_id(&category_view.category.id.to_string()).await {
                category_view.products = products;
            }

            return commercyfy_success!(category_view);
        }
    }

    return commercyfy_fail!(
        StatusCode::NOT_FOUND,
        format!("Category with id '{id}' not found")
    );
}
