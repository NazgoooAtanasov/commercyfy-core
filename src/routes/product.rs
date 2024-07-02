use std::collections::HashMap;

use super::{CommercyfyResponse, CreatedEntryResponse};
use crate::models::base_extensions::FieldExtensionObject;
use crate::models::category::Category;
use crate::models::inventory::ProductInventoryRecord;
use crate::models::portal_user::{JWTClaims, PortalUsersRoles};
use crate::models::pricebook::PricebookRecord;
use crate::models::product::ProductImage;
use crate::schemas::product::{CreateProduct, CreateProductImage};
use crate::services::unstructureddb::entry::UnstructuredEntryType;
use crate::services::{
    db::DbService, role_validation::RoleService, unstructureddb::UnstructuredDb,
};
use crate::utils::custom_fields::create_custom_fields;
use crate::{models::product::Product, CommercyfyExtrState};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};

pub async fn get_products(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
) -> CommercyfyResponse<Vec<Product>> {
    if let Err(err) = state.role_service.validate_any(
        &claims,
        vec![PortalUsersRoles::ADMIN, PortalUsersRoles::READER],
    ) {
        return commercyfy_fail!(err);
    }

    let products = match state.db_service.get_products().await {
        Ok(products) => products,
        Err(error) => return commercyfy_fail!(error.to_string()),
    };

    return commercyfy_success!(products);
}

#[derive(serde::Serialize)]
pub struct ProductView {
    #[serde(flatten)]
    product: Product,
    images: Vec<ProductImage>,
    custom_fields: HashMap<String, UnstructuredEntryType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<Vec<Category>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    inventories: Option<Vec<ProductInventoryRecord>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pricebooks: Option<Vec<PricebookRecord>>,
}

pub async fn get_product(
    Query(params): Query<HashMap<String, String>>,
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
    Path(id): Path<String>,
) -> CommercyfyResponse<ProductView> {
    if let Err(err) = state.role_service.validate_any(
        &claims,
        vec![PortalUsersRoles::ADMIN, PortalUsersRoles::READER],
    ) {
        return commercyfy_fail!(err);
    }

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
        categories: None,
        inventories: None,
        pricebooks: None,
        custom_fields: HashMap::new(),
    };

    let images_check = state.db_service.get_product_images(&id).await;
    if let Ok(images) = images_check {
        product_view.images = images;
    }

    if let Ok(custom_fields) = state
        .unstructureddb
        .get_custom_fields(
            FieldExtensionObject::PRODUCT,
            &product_view.product.id.to_string(),
        )
        .await
    {
        for field in custom_fields {
            product_view
                .custom_fields
                .insert(field.field_name, field.value);
        }
    }

    if let Some(value) = params.get("extend") {
        if value.contains("categories") {
            let categories = match state
                .db_service
                .get_product_categories(&product_view.product.id.to_string())
                .await
            {
                Ok(categories) => categories,
                Err(err) => return commercyfy_fail!(err.to_string()),
            };

            product_view.categories = Some(categories);
        }

        if value.contains("inventories") {
            let records = match state
                .db_service
                .get_product_inventory_records(&product_view.product.id.to_string())
                .await
            {
                Ok(records) => records,
                Err(err) => return commercyfy_fail!(err.to_string()),
            };

            product_view.inventories = Some(records);
        }

        if value.contains("pricebooks") {
            let pricebooks = match state
                .db_service
                .get_product_pricebooks(&product_view.product.id.to_string())
                .await
            {
                Ok(pricebooks) => pricebooks,
                Err(err) => return commercyfy_fail!(err.to_string()),
            };

            product_view.pricebooks = Some(pricebooks);
        }
    }

    return commercyfy_success!(product_view);
}

pub async fn create_product(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreateProduct>,
) -> CommercyfyResponse<CreatedEntryResponse> {
    if let Err(err) = state.role_service.validate_any(
        &claims,
        vec![PortalUsersRoles::ADMIN, PortalUsersRoles::EDITOR],
    ) {
        return commercyfy_fail!(err);
    }

    if let Err(error) = payload.validate() {
        return commercyfy_fail!(error.to_string());
    }

    let category_assignments = payload.category_assignments.clone();
    let product_create = state.db_service.create_product(&payload).await;
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

    if let Err(err) = create_custom_fields(
        state,
        product.id.to_string(),
        FieldExtensionObject::PRODUCT,
        &payload.custom_fields,
    )
    .await
    {
        return commercyfy_fail!(err);
    }

    return commercyfy_success!(StatusCode::CREATED, CreatedEntryResponse { id: product.id });
}

pub async fn create_product_image(
    Extension(claims): Extension<JWTClaims>,
    Path(id): Path<String>,
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreateProductImage>,
) -> CommercyfyResponse<CreatedEntryResponse> {
    if let Err(err) = state.role_service.validate_any(
        &claims,
        vec![PortalUsersRoles::ADMIN, PortalUsersRoles::EDITOR],
    ) {
        return commercyfy_fail!(err);
    }

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
