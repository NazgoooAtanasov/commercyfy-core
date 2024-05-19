use super::{CommercyfyResponse, CreatedEntryResponse};
use crate::{
    models::pricebook::{Pricebook, PricebookRecord},
    schemas::pricebook::{CreatePricebook, CreatePricebookRecord},
    services::db::DbService,
    CommercyfyExtrState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn get_pricebooks(
    State(state): CommercyfyExtrState,
) -> CommercyfyResponse<Vec<Pricebook>> {
    let pricebooks = state.db_service.get_pricebooks().await;
    if let Err(error) = pricebooks {
        return commercyfy_fail!(error.to_string());
    }

    return commercyfy_success!(pricebooks.unwrap());
}

pub async fn get_pricebook(
    State(state): CommercyfyExtrState,
    Path(id): Path<String>,
) -> CommercyfyResponse<Pricebook> {
    let pricebook = state.db_service.get_pricebook_by_id(&id).await;
    if let Err(err) = pricebook {
        return commercyfy_fail!(err.to_string());
    }
    if let Some(pricebook) = pricebook.unwrap() {
        return commercyfy_success!(pricebook);
    }

    let pricebook = state.db_service.get_pricebook_by_reference(&id).await;
    if let Err(err) = pricebook {
        return commercyfy_fail!(err.to_string());
    }
    if let Some(pricebook) = pricebook.unwrap() {
        return commercyfy_success!(pricebook);
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
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreatePricebook>,
) -> CommercyfyResponse<CreatedEntryResponse> {
    if let Err(err) = payload.validate() {
        return commercyfy_fail!(err);
    }

    let pricebook_creation = state.db_service.create_pricebook(payload).await;
    if let Err(err) = pricebook_creation {
        return commercyfy_fail!(err.to_string());
    }

    return commercyfy_success!(
        StatusCode::CREATED,
        CreatedEntryResponse {
            id: pricebook_creation.unwrap().id
        }
    );
}

pub async fn create_pricebook_record(
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreatePricebookRecord>,
) -> CommercyfyResponse<CreatedEntryResponse> {
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
    State(state): CommercyfyExtrState,
    Path(path): Path<(String, String)>
) -> CommercyfyResponse<PricebookRecord> {
    let (pricebook_id, product_id) = path;

    let pricebook_record = state.db_service.get_product_pricebook_record(&product_id, &pricebook_id).await;

    if let Err(err) = pricebook_record {
        return commercyfy_fail!(err.to_string());
    }

    if let Some(pricebook_record) = pricebook_record.unwrap() {
        return commercyfy_success!(pricebook_record);
    }

    return commercyfy_fail!(StatusCode::NOT_FOUND, format!("There is no pricebook record with the provided ids."));
}
