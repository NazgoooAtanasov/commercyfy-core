use super::{CommercyfyResponse, CreatedEntryResponse};
use crate::{
    models::base_extensions::{FieldExtension, FieldExtensionObject},
    schemas::base_extensions::CreateCustomField,
    services::db::DbService,
    CommercyfyExtrState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn create_extension(
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreateCustomField>,
) -> CommercyfyResponse<CreatedEntryResponse> {
    if let Err(err) = payload.validate() {
        return commercyfy_fail!(err);
    }

    let existing_custom_field = match state
        .db_service
        .get_custom_field(payload.object.clone(), &payload.base_felds.name)
        .await
    {
        Ok(field) => field,
        Err(err) => return commercyfy_fail!(err.to_string()),
    };

    if let Some(_) = existing_custom_field {
        return commercyfy_fail!(
            "Field with that name already exists on the provided object type".to_string()
        );
    }

    let custom_field = match state.db_service.create_custom_field(payload).await {
        Ok(field) => field,
        Err(err) => return commercyfy_fail!(err.to_string()),
    };

    return commercyfy_success!(
        StatusCode::CREATED,
        CreatedEntryResponse {
            id: custom_field.id
        }
    );
}

pub async fn get_extensions(
    State(state): CommercyfyExtrState,
    Path(object_type): Path<String>,
) -> CommercyfyResponse<Vec<FieldExtension>> {
    let object = match object_type.to_lowercase().as_str() {
        "product" => FieldExtensionObject::PRODUCT,
        _ => {
            return commercyfy_fail!(
                StatusCode::NOT_FOUND,
                format!("There is no object type '{}'", object_type)
            )
        }
    };

    let extensions = state.db_service.get_custom_fields(object).await;

    if let Err(err) = extensions {
        return commercyfy_fail!(err.to_string());
    }

    return commercyfy_success!(extensions.unwrap());
}
