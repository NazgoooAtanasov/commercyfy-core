use axum::{
    async_trait,
    body::Bytes,
    extract::{rejection::BytesRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use serde_valid::{validation::error::Errors, Validate};

pub struct StructuredJson<T>(pub T);

#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum StrucutedJsonRejectionErrors {
    String(String),
    ValidationError(Errors),
}

#[derive(serde::Serialize)]
pub struct StructuredJsonRejection {
    error: StrucutedJsonRejectionErrors,
}
impl IntoResponse for StructuredJsonRejection {
    fn into_response(self) -> Response {
        let err_body = match serde_json::to_string(&self) {
            Ok(str) => str,
            Err(_) => String::from("{\"error\":\"Internal Server Error\"}"),
        };
        let response = Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-type", "application/json")
            .body(err_body)
            .unwrap()
            .into_response();
        return response;
    }
}
impl From<BytesRejection> for StructuredJsonRejection {
    fn from(_: BytesRejection) -> Self {
        return StructuredJsonRejection {
            error: StrucutedJsonRejectionErrors::String(String::from("Invalid JSON")),
        };
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for StructuredJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = StructuredJsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state).await?;

        return match serde_json::from_slice::<T>(&body) {
            Ok(object) => {
                if let Err(err) = object.validate() {
                    return Err(StructuredJsonRejection {
                        error: StrucutedJsonRejectionErrors::ValidationError(err),
                    });
                }

                Ok(StructuredJson(object))
            }
            Err(error) => Err(StructuredJsonRejection {
                error: StrucutedJsonRejectionErrors::String(error.to_string()),
            }),
        };
    }
}
