use axum::{extract::Request, http, http::StatusCode, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::models::portal_user::JWTClaims;

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = if let Some(auth_header) = req.headers().get(http::header::AUTHORIZATION) {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if auth_header.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let auth_header_str = if let Ok(string) = auth_header.to_str() {
        string
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let token = if let Some(token) = auth_header_str.split(" ").skip(1).take(1).nth(0) {
        token
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let jwt_secret = std::env::var("JWT_TOKEN_SECRET");
    if let Err(_err) = &jwt_secret {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let claims = if let Ok(claims) = decode::<JWTClaims>(
        &token,
        &DecodingKey::from_secret(&jwt_secret.unwrap().as_bytes()),
        &Validation::default(),
    ) {
        claims
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    req.extensions_mut().insert(claims.claims);

    return Ok(next.run(req).await);
}
