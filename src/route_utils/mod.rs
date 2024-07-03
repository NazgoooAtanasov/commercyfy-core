use axum::Json;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum CommercyfyResponseData<T: serde::Serialize> {
    Success(T),
    Error { error: String },
}
pub type CommercyfyResponse<T> = (axum::http::StatusCode, Json<CommercyfyResponseData<T>>);

#[macro_export]
macro_rules! commercyfy_success {
    ($x: expr) => {
        (
            axum::http::StatusCode::OK,
            axum::Json(commercyfy_core::route_utils::CommercyfyResponseData::Success($x)),
        )
    };

    ($y: expr, $x: expr) => {
        (
            $y,
            axum::Json(commercyfy_core::route_utils::CommercyfyResponseData::Success($x)),
        )
    };
}

#[macro_export]
macro_rules! commercyfy_fail {
    ($x: expr) => {
        (
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(commercyfy_core::route_utils::CommercyfyResponseData::Error { error: $x }),
        )
    };

    ($y: expr, $x: expr) => {
        (
            $y,
            axum::Json(commercyfy_core::route_utils::CommercyfyResponseData::Error { error: $x }),
        )
    };
}
