use super::{CommercyfyResponse, CreatedEntryResponse};
use crate::models::product::ProductImage;
use crate::schemas::product::{CreateProduct, CreateProductImage};
use crate::services::db::DbService;
use crate::{models::product::Product, CommercyfyExtrState};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

#[derive(serde::Serialize)]
pub struct ProductView {
    #[serde(flatten)]
    product: Product,
    images: Vec<ProductImage>,
}

pub async fn get_product(
    State(state): CommercyfyExtrState,
    Path(id): Path<String>,
) -> CommercyfyResponse<ProductView> {
    let product_check = state.db_service.get_product(&id).await;

    if let Err(error) = product_check {
        return commercyfy_fail!(error.to_string());
    }

    let product = product_check.unwrap();
    if let None = product {
        return commercyfy_fail!(
            StatusCode::NOT_FOUND,
            format!("Product with 'id' '{id}' is not found")
        );
    }

    let mut product_view = ProductView {
        product: product.unwrap(),
        images: Vec::new(),
    };

    let images_check = state.db_service.get_product_images(&id).await;
    if let Ok(images) = images_check {
        product_view.images = images;
    }

    return commercyfy_success!(product_view);
}

pub async fn create_product(
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreateProduct>,
) -> CommercyfyResponse<CreatedEntryResponse> {
    if let Err(error) = payload.validate() {
        return commercyfy_fail!(error.to_string());
    }

    let category_assignments = payload.category_assignments.clone();
    let product_create = state.db_service.create_product(payload).await;
    if let Err(err) = product_create {
        return commercyfy_fail!(err.to_string());
    }

    let product = product_create.unwrap();
    if let Some(categories) = category_assignments {
        if let Err(error) = state
            .db_service
            .create_product_category_assignment(product.id, categories)
            .await
        {
            return commercyfy_fail!(format!("Assignment failed because: {}, but the cretion of the product should be successfull", error.to_string()));
        }
    }

    return commercyfy_success!(StatusCode::CREATED, CreatedEntryResponse { id: product.id });
}

pub async fn create_product_image(
    Path(id): Path<String>,
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreateProductImage>,
) -> CommercyfyResponse<CreatedEntryResponse> {
    if let Err(error) = payload.validate() {
        return commercyfy_fail!(error);
    }

    let product_check = state.db_service.get_product(&id).await;
    if let Err(error) = product_check {
        return commercyfy_fail!(error.to_string());
    }

    if let None = product_check.unwrap() {
        return commercyfy_fail!(
            StatusCode::NOT_FOUND,
            format!("Product with 'id' '{id}' is not found")
        );
    }

    let create_check = state.db_service.create_product_image(&id, payload).await;
    if let Err(error) = create_check {
        return commercyfy_fail!(error.to_string());
    }

    return commercyfy_success!(CreatedEntryResponse {
        id: create_check.unwrap().id
    });
}
