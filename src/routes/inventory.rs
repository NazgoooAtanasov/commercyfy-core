use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};

use super::{CommercyfyResponse, CreatedEntryResponse};
use crate::models::base_extensions::FieldExtensionObject;
use crate::models::inventory::ProductInventoryRecord;
use crate::models::portal_user::{JWTClaims, PortalUsersRoles};
use crate::schemas::inventory::{CreateInventory, CreateInventoryRecord};
use crate::services::db::DbService;
use crate::services::role_validation::RoleService;
use crate::services::unstructureddb::entry::UnstructuredEntryType;
use crate::services::unstructureddb::UnstructuredDb;
use crate::utils::custom_fields::create_custom_fields;
use crate::{models::inventory::Inventory, CommercyfyExtrState};

pub async fn get_inventories(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
) -> CommercyfyResponse<Vec<Inventory>> {
    if let Err(err) = state.role_service.validate_any(
        &claims,
        vec![PortalUsersRoles::ADMIN, PortalUsersRoles::READER],
    ) {
        return commercyfy_fail!(err);
    }

    let inventories = state.db_service.get_inventories().await;
    if let Err(error) = inventories {
        return commercyfy_fail!(error.to_string());
    }

    return commercyfy_success!(inventories.unwrap());
}

#[derive(serde::Serialize)]
pub struct InventoryView {
    #[serde(flatten)]
    inventory: Inventory,
    records: Vec<ProductInventoryRecord>,
    custom_fields: HashMap<String, UnstructuredEntryType>,
}
pub async fn get_inventory(
    Extension(claims): Extension<JWTClaims>,
    Path(id): Path<String>,
    State(state): CommercyfyExtrState,
) -> CommercyfyResponse<InventoryView> {
    if let Err(err) = state.role_service.validate_any(
        &claims,
        vec![PortalUsersRoles::ADMIN, PortalUsersRoles::READER],
    ) {
        return commercyfy_fail!(err);
    }

    let inventory_id_check = state.db_service.get_inventory_by_id(&id).await;
    if let Err(error) = inventory_id_check {
        return commercyfy_fail!(error.to_string());
    }

    let inventory = inventory_id_check.unwrap();
    if let Some(inventory) = inventory {
        let mut inventory_view = InventoryView {
            inventory,
            records: Vec::new(),
            custom_fields: HashMap::new(),
        };

        let records_check = state
            .db_service
            .get_inventory_records(&inventory_view.inventory.id.to_string())
            .await;

        if let Ok(records) = records_check {
            inventory_view.records = records;
        }

        if let Ok(custom_fields) = state
            .unstructureddb
            .get_custom_fields(
                FieldExtensionObject::INVENTORY,
                &inventory_view.inventory.id.to_string(),
            )
            .await
        {
            for field in custom_fields {
                inventory_view
                    .custom_fields
                    .insert(field.field_name, field.value);
            }
        }

        return commercyfy_success!(inventory_view);
    }

    let inventory_reference_check = state.db_service.get_inventory_by_reference(&id).await;
    if let Err(error) = inventory_reference_check {
        return commercyfy_fail!(error.to_string());
    }

    let inventory = inventory_reference_check.unwrap();
    if let Some(inventory) = inventory {
        let mut inventory_view = InventoryView {
            inventory,
            records: Vec::new(),
            custom_fields: HashMap::new(),
        };

        if let Ok(records) = state
            .db_service
            .get_inventory_records(&inventory_view.inventory.id.to_string())
            .await
        {
            inventory_view.records = records;
        }

        if let Ok(custom_fields) = state
            .unstructureddb
            .get_custom_fields(
                FieldExtensionObject::INVENTORY,
                &inventory_view.inventory.id.to_string(),
            )
            .await
        {
            for field in custom_fields {
                inventory_view
                    .custom_fields
                    .insert(field.field_name, field.value);
            }
        }

        return commercyfy_success!(inventory_view);
    }

    return commercyfy_fail!(
        StatusCode::NOT_FOUND,
        format!("Inventory with the provided id or reference could not be found")
    );
}

pub async fn create_inventory(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreateInventory>,
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

    let exists = state
        .db_service
        .get_inventory_by_reference(&payload.inventory_reference)
        .await;
    if let Err(error) = exists {
        return commercyfy_fail!(error.to_string());
    }

    let inventory_check = exists.unwrap();
    if let Some(_) = inventory_check {
        return commercyfy_fail!(format!("Inventory with that reference already exists"));
    }

    let inventory = match state.db_service.create_inventory(&payload).await {
        Ok(inv) => inv,
        Err(error) => return commercyfy_fail!(error.to_string()),
    };

    if let Err(err) = create_custom_fields(
        state,
        inventory.id.to_string(),
        FieldExtensionObject::INVENTORY,
        &payload.custom_fields,
    )
    .await
    {
        return commercyfy_fail!(err.to_string());
    }

    return commercyfy_success!(CreatedEntryResponse { id: inventory.id });
}

pub async fn create_inventory_record(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreateInventoryRecord>,
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

    let inventory_check = state
        .db_service
        .get_inventory_by_id(&payload.inventory_id.to_string())
        .await;
    if let Err(error) = inventory_check {
        return commercyfy_fail!(error.to_string());
    }

    let inventory = inventory_check.unwrap();
    if let None = inventory {
        return commercyfy_fail!(format!(
            "Inventory with reference '{}' does not exist",
            payload.inventory_id
        ));
    }

    let product_check = state
        .db_service
        .get_product(&payload.product_id.to_string())
        .await;
    if let Err(error) = product_check {
        return commercyfy_fail!(error.to_string());
    }

    let product = product_check.unwrap();
    if let None = product {
        return commercyfy_fail!(format!(
            "Product with id '{}' does not exist",
            payload.product_id.to_string()
        ));
    }

    let record_exists = state
        .db_service
        .get_product_inventory_record(
            &payload.product_id.to_string(),
            &payload.inventory_id.to_string(),
        )
        .await;
    if let Err(error) = record_exists {
        return commercyfy_fail!(error.to_string());
    }
    if let Some(_) = record_exists.unwrap() {
        return commercyfy_fail!(format!(
            "Record for the provided product already exists in the provided inventory"
        ));
    }

    let record_check = state
        .db_service
        .create_product_inventory_record(payload)
        .await;
    if let Err(err) = record_check {
        return commercyfy_fail!(err.to_string());
    }

    return commercyfy_success!(
        StatusCode::CREATED,
        CreatedEntryResponse {
            id: record_check.unwrap().id
        }
    );
}

pub async fn get_inventory_record(
    Extension(claims): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
    Path(path): Path<(String, String)>,
) -> CommercyfyResponse<ProductInventoryRecord> {
    if let Err(err) = state.role_service.validate_any(
        &claims,
        vec![PortalUsersRoles::ADMIN, PortalUsersRoles::READER],
    ) {
        return commercyfy_fail!(err);
    }

    let (inventory_id, product_id) = path;
    let record_check = state
        .db_service
        .get_product_inventory_record(&product_id, &inventory_id)
        .await;
    if let Err(error) = record_check {
        return commercyfy_fail!(error.to_string());
    }

    let record = record_check.unwrap();
    if let None = record {
        return commercyfy_fail!(StatusCode::NOT_FOUND, format!("No record was found"));
    }

    return commercyfy_success!(record.unwrap());
}
