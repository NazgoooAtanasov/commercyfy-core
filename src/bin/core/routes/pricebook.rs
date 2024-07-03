use std::collections::HashMap;

use super::{CommercyfyExtrState, CreatedEntryResponse};
use crate::utils::custom_fields::create_custom_fields;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use commercyfy_core::{
    commercyfy_fail, commercyfy_success, models::{
        base_extensions::FieldExtensionObject,
        portal_user::{JWTClaims, PortalUsersRoles},
        pricebook::{Pricebook, PricebookRecord},
    }, route_utils::CommercyfyResponse, schemas::pricebook::{CreatePricebook, CreatePricebookRecord}, services::{
        db::DbService,
        role_validation::RoleService,
        unstructureddb::{entry::UnstructuredEntryType, UnstructuredDb},
    }
};

pub async fn get_pricebooks(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
) -> CommercyfyResponse<Vec<Pricebook>> {
    if let Err(err) = state.role_service.validate_any(
        &claims,
        vec![PortalUsersRoles::ADMIN, PortalUsersRoles::READER],
    ) {
        return commercyfy_fail!(err);
    }

    let pricebooks = state.db_service.get_pricebooks().await;
    if let Err(error) = pricebooks {
        return commercyfy_fail!(error.to_string());
    }

    return commercyfy_success!(pricebooks.unwrap());
}

#[derive(serde::Serialize)]
pub struct PricebookView {
    #[serde(flatten)]
    pricebook: Pricebook,
    records: Vec<PricebookRecord>,
    custom_fields: HashMap<String, UnstructuredEntryType>,
}
pub async fn get_pricebook(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
    Path(id): Path<String>,
) -> CommercyfyResponse<PricebookView> {
    if let Err(err) = state.role_service.validate_any(
        &claims,
        vec![PortalUsersRoles::ADMIN, PortalUsersRoles::READER],
    ) {
        return commercyfy_fail!(err);
    }

    let pricebook = state.db_service.get_pricebook_by_id(&id).await;
    if let Err(err) = pricebook {
        return commercyfy_fail!(err.to_string());
    }
    if let Some(pricebook) = pricebook.unwrap() {
        let mut pricebook_view = PricebookView {
            pricebook,
            custom_fields: HashMap::new(),
            records: vec![],
        };

        if let Ok(records) = state
            .db_service
            .get_pricebook_records(&pricebook_view.pricebook.id.to_string())
            .await
        {
            pricebook_view.records = records;
        }

        if let Ok(custom_fields) = state
            .unstructureddb
            .get_custom_fields(
                FieldExtensionObject::PRICEBOOK,
                &pricebook_view.pricebook.id.to_string(),
            )
            .await
        {
            for field in custom_fields {
                pricebook_view
                    .custom_fields
                    .insert(field.field_name, field.value);
            }
        }

        return commercyfy_success!(pricebook_view);
    }

    let pricebook = state.db_service.get_pricebook_by_reference(&id).await;
    if let Err(err) = pricebook {
        return commercyfy_fail!(err.to_string());
    }
    if let Some(pricebook) = pricebook.unwrap() {
        let mut pricebook_view = PricebookView {
            pricebook,
            custom_fields: HashMap::new(),
            records: vec![],
        };

        if let Ok(records) = state
            .db_service
            .get_pricebook_records(&pricebook_view.pricebook.id.to_string())
            .await
        {
            pricebook_view.records = records;
        }

        if let Ok(custom_fields) = state
            .unstructureddb
            .get_custom_fields(
                FieldExtensionObject::PRICEBOOK,
                &pricebook_view.pricebook.id.to_string(),
            )
            .await
        {
            for field in custom_fields {
                pricebook_view
                    .custom_fields
                    .insert(field.field_name, field.value);
            }
        }

        return commercyfy_success!(pricebook_view);
    }

    return commercyfy_fail!(
        StatusCode::NOT_FOUND,
        format!(
            "Pricebook with the provided, {}, id/reference was not found",
            id
        )
    );
}

pub async fn create_pricebook(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreatePricebook>,
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

    let pricebook_creation = match state.db_service.create_pricebook(&payload).await {
        Ok(pricebook) => pricebook,
        Err(err) => return commercyfy_fail!(err.to_string()),
    };

    if let Err(err) = create_custom_fields(
        state,
        pricebook_creation.id.to_string(),
        FieldExtensionObject::PRICEBOOK,
        &payload.custom_fields,
    )
    .await
    {
        return commercyfy_fail!(err);
    }

    return commercyfy_success!(
        StatusCode::CREATED,
        CreatedEntryResponse {
            id: pricebook_creation.id
        }
    );
}

pub async fn create_pricebook_record(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreatePricebookRecord>,
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

    let product = state.db_service.get_product(&payload.product_id).await;
    if let Err(err) = product {
        return commercyfy_fail!(err.to_string());
    }
    if let None = product.unwrap() {
        return commercyfy_fail!(format!(
            "Product with id '{}' was not found.",
            payload.product_id
        ));
    }

    let pricebook = state
        .db_service
        .get_pricebook_by_id(&payload.pricebook_id)
        .await;
    if let Err(err) = pricebook {
        return commercyfy_fail!(err.to_string());
    }
    if let None = pricebook.unwrap() {
        return commercyfy_fail!(format!(
            "Pricebook with id '{}' was not found.",
            payload.pricebook_id
        ));
    }

    let pricebook_record = state
        .db_service
        .create_product_pricebook_record(payload)
        .await;
    if let Err(err) = pricebook_record {
        return commercyfy_fail!(err.to_string());
    }

    return commercyfy_success!(
        StatusCode::CREATED,
        CreatedEntryResponse {
            id: pricebook_record.unwrap().id
        }
    );
}

pub async fn get_pricebook_record(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
    Path(path): Path<(String, String)>,
) -> CommercyfyResponse<PricebookRecord> {
    if let Err(err) = state.role_service.validate_any(
        &claims,
        vec![PortalUsersRoles::ADMIN, PortalUsersRoles::READER],
    ) {
        return commercyfy_fail!(err);
    }

    let (pricebook_id, product_id) = path;

    let pricebook_record = state
        .db_service
        .get_product_pricebook_record(&product_id, &pricebook_id)
        .await;

    if let Err(err) = pricebook_record {
        return commercyfy_fail!(err.to_string());
    }

    if let Some(pricebook_record) = pricebook_record.unwrap() {
        return commercyfy_success!(pricebook_record);
    }

    return commercyfy_fail!(
        StatusCode::NOT_FOUND,
        format!("There is no pricebook record with the provided ids.")
    );
}
