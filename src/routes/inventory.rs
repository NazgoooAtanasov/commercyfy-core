use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

use super::{CommercyfyResponse, CreatedEntryResponse};
use crate::models::inventory::ProductInventoryRecord;
use crate::schemas::inventory::{CreateInventory, CreateInventoryRecord};
use crate::services::db::DbService;
use crate::{models::inventory::Inventory, CommercyfyExtrState};

pub async fn get_inventories(
    State(state): CommercyfyExtrState,
) -> CommercyfyResponse<Vec<Inventory>> {
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
}
pub async fn get_inventory(
    Path(id): Path<String>,
    State(state): CommercyfyExtrState,
) -> CommercyfyResponse<InventoryView> {
    let inventory_id_check = state.db_service.get_inventory_by_id(&id).await;
    if let Err(error) = inventory_id_check {
        return commercyfy_fail!(error.to_string());
    }

    let inventory = inventory_id_check.unwrap();
    if let Some(inventory) = inventory {
        let mut inventory_view = InventoryView {
            inventory,
            records: Vec::new(),
        };

        let records_check = state
            .db_service
            .get_inventory_records(&inventory_view.inventory.id.to_string())
            .await;

        if let Ok(records) = records_check {
            inventory_view.records = records;
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
        };

        if let Ok(records) = state
            .db_service
            .get_inventory_records(&inventory_view.inventory.id.to_string())
            .await
        {
            inventory_view.records = records;
        }

        return commercyfy_success!(inventory_view);
    }

    return commercyfy_fail!(
        StatusCode::NOT_FOUND,
        format!("Inventory with the provided id or reference could not be found")
    );
}

pub async fn create_inventory(
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreateInventory>,
) -> CommercyfyResponse<CreatedEntryResponse> {
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

    let inventory = exists.unwrap();
    if let Some(_) = inventory {
        return commercyfy_fail!(format!("Inventory with that reference already exists"));
    }

    let inventory = state.db_service.create_inventory(payload).await;
    if let Err(error) = inventory {
        return commercyfy_fail!(error.to_string());
    }

    return commercyfy_success!(CreatedEntryResponse {
        id: inventory.unwrap().id
    });
}

pub async fn create_inventory_record(
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreateInventoryRecord>,
) -> CommercyfyResponse<CreatedEntryResponse> {
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
    State(state): CommercyfyExtrState,
    Path(path): Path<(String, String)>,
) -> CommercyfyResponse<ProductInventoryRecord> {
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
