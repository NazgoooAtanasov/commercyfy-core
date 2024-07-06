use std::collections::HashMap;

use super::CommercyfyExtrState;
use crate::utils::custom_fields::create_custom_fields;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use commercyfy_core::{
    commercyfy_fail, commercyfy_success,
    models::{
        base_extensions::FieldExtensionObject,
        category::Category,
        portal_user::{JWTClaims, PortalUsersRoles},
        product::Product,
    },
    route_utils::{CommercyfyResponse, CreatedEntryResponse},
    schemas::category::{AssignProductToCategory, CreateCategory},
    services::{
        db::DbService,
        role_validation::RoleService,
        unstructureddb::{entry::UnstructuredEntryType, UnstructuredDb},
    },
};

pub async fn get_categories(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
) -> CommercyfyResponse<Vec<Category>> {
    if let Err(err) = state.role_service.validate_any(
        &claims,
        vec![PortalUsersRoles::ADMIN, PortalUsersRoles::READER],
    ) {
        return commercyfy_fail!(err);
    }

    let categories = state.db_service.get_categories().await;

    if let Err(error) = categories {
        return commercyfy_fail!(error.to_string());
    }

    return commercyfy_success!(categories.unwrap());
}

pub async fn create_category(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreateCategory>,
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

    let created = state.db_service.create_category(&payload).await;
    if let Err(error) = created {
        return commercyfy_fail!(error.to_string());
    }

    let category = created.unwrap();

    if let Err(err) = create_custom_fields(
        state,
        category.id.to_string(),
        FieldExtensionObject::CATEGORY,
        &payload.custom_fields,
    )
    .await
    {
        return commercyfy_fail!(err);
    }

    return commercyfy_success!(
        StatusCode::CREATED,
        CreatedEntryResponse { id: category.id }
    );
}

#[derive(serde::Serialize)]
pub struct CategoryView {
    #[serde(flatten)]
    category: Category,
    products: Vec<Product>,
    custom_fields: HashMap<String, UnstructuredEntryType>,
}
pub async fn get_category(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
    Path(id): Path<String>,
) -> CommercyfyResponse<CategoryView> {
    if let Err(err) = state.role_service.validate_any(
        &claims,
        vec![PortalUsersRoles::ADMIN, PortalUsersRoles::READER],
    ) {
        return commercyfy_fail!(err);
    }

    if let Ok(category) = state.db_service.get_category_by_id(&id).await {
        if let Some(cat) = category {
            let mut category_view = CategoryView {
                category: cat,
                products: Vec::new(),
                custom_fields: HashMap::new(),
            };

            if let Ok(products) = state.db_service.get_category_products_by_id(&id).await {
                category_view.products = products;
            }

            if let Ok(custom_fields) = state
                .unstructureddb
                .get_custom_fields(
                    FieldExtensionObject::CATEGORY,
                    &category_view.category.id.to_string(),
                )
                .await
            {
                for entry in custom_fields {
                    category_view
                        .custom_fields
                        .insert(entry.field_name, entry.value);
                }
            };

            return commercyfy_success!(category_view);
        }
    }

    if let Ok(category) = state.db_service.get_category_by_reference(&id).await {
        if let Some(cat) = category {
            let mut category_view = CategoryView {
                category: cat,
                products: Vec::new(),
                custom_fields: HashMap::new(),
            };

            if let Ok(products) = state
                .db_service
                .get_category_products_by_id(&category_view.category.id.to_string())
                .await
            {
                category_view.products = products;
            }

            if let Ok(custom_fields) = state
                .unstructureddb
                .get_custom_fields(
                    FieldExtensionObject::CATEGORY,
                    &category_view.category.id.to_string(),
                )
                .await
            {
                for entry in custom_fields {
                    category_view
                        .custom_fields
                        .insert(entry.field_name, entry.value);
                }
            };

            return commercyfy_success!(category_view);
        }
    }

    return commercyfy_fail!(
        StatusCode::NOT_FOUND,
        format!("Category with id '{id}' not found")
    );
}

pub async fn assign_products_to_category(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
    Json(payload): Json<AssignProductToCategory>,
) -> CommercyfyResponse<CreatedEntryResponse> {
    if let Err(err) = state.role_service.validate_any(
        &claims,
        vec![PortalUsersRoles::ADMIN, PortalUsersRoles::EDITOR],
    ) {
        return commercyfy_fail!(err);
    }

    if let Err(err) = payload.validate() {
        return commercyfy_fail!(err);
    }

    match state
        .db_service
        .create_category_product_entries(&payload)
        .await
    {
        Ok(_) => {
            return commercyfy_success!(
                StatusCode::CREATED,
                CreatedEntryResponse {
                    id: uuid::uuid!("00000000-0000-0000-0000-000000000000")
                }
            )
        }
        Err(err) => return commercyfy_fail!(err.to_string()),
    };
}
