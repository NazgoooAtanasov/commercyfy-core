use axum::Json;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum CommercyfyResponseData<T: serde::Serialize> {
    Success(T),
    Error { error: String },
}
pub type CommercyfyResponse<T> = (axum::http::StatusCode, Json<CommercyfyResponseData<T>>);

#[derive(serde::Serialize, Debug)]
pub struct CreatedEntryResponse {
    pub id: sqlx::types::Uuid
}

#[macro_export]
macro_rules! commercyfy_success {
    ($x: expr) => {
        (StatusCode::OK, Json(CommercyfyResponseData::Success($x)))
    };

    ($y: expr, $x: expr) => {
        ($y, Json(CommercyfyResponseData::Success($x)))
    }
}

#[macro_export]
macro_rules! commercyfy_fail {
    ($x: expr) => {
        (StatusCode::BAD_REQUEST, Json(CommercyfyResponseData::Error{error: $x}))
    };

    ($y: expr, $x: expr) => {
        ($y, Json(CommercyfyResponseData::Error{error: $x}))
    }
}

pub mod category;
// pub mod inventory;
// pub mod portal_user;
// pub mod pricebook;
// pub mod product;

// pub mod base_extensions;
