use std::time::{self, Duration, SystemTime};

use crate::{models::portal_user::JWTClaims, services::{db::DbService, role_validation::RoleService}};
use argon2::{PasswordHash, PasswordVerifier};
use axum::{extract::{Path, State}, http::StatusCode, Extension, Json};
use jsonwebtoken::{EncodingKey, Header};
use crate::{models::portal_user::{PortalUser, SignInToken}, schemas::portal_user::{PortalUserCreate, PortalUserSignin}, CommercyfyExtrState};
use super::{CommercyfyResponse, CreatedEntryResponse};

pub async fn get_portal_user(
    Extension(claim): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
    Path(id): Path<String>
) -> CommercyfyResponse<PortalUser> {
    if let Err(err) = state.role_service.validate_admin(&claim) {
        return commercyfy_fail!(err);
    }

    let portal_user = state.db_service.get_portal_user(&id).await;
    if let Err(err) = portal_user {
        return commercyfy_fail!(err.to_string());
    }

    if let Some(portal_user) = portal_user.unwrap() {
        return commercyfy_success!(portal_user);
    }

    return commercyfy_fail!(StatusCode::NOT_FOUND, format!("Portal user with id '{}' was not found", id));
}

pub async fn create_portal_user(
    Extension(claim): Extension<JWTClaims>,
    State(state): CommercyfyExtrState,
    Json(payload): Json<PortalUserCreate>
) -> CommercyfyResponse<CreatedEntryResponse> {
    if let Err(err) = state.role_service.validate_admin(&claim) {
        return commercyfy_fail!(err);
    }

    if let Err(err) = payload.validate() {
        return commercyfy_fail!(err);
    }

    let existing = state.db_service.get_portal_user_by_email(&payload.email).await;
    if let Err(err) = existing {
        return commercyfy_fail!(err.to_string());
    }
    if let Some(_) = existing.unwrap() {
        return commercyfy_fail!("User with that email already exists".to_string());
    }

    let user = state.db_service.create_portal_user(payload).await;
    if let Err(err) = user {
        match err {
            sqlx::Error::Io(_) => return commercyfy_fail!("There was an error creating a portal_user".to_string()),
            anyerr => return commercyfy_fail!(anyerr.to_string()),
        }
    }

    return commercyfy_success!(StatusCode::CREATED, CreatedEntryResponse { id: user.unwrap().id });
}

pub async fn signin_portal_user(
    State(state): CommercyfyExtrState,
    Json(payload): Json<PortalUserSignin>
) -> CommercyfyResponse<SignInToken> {
    if let Err(err) = payload.validate() {
        return commercyfy_fail!(err);
    }

    let user_check = state.db_service.get_portal_user_by_email(&payload.email).await;
    if let Err(err) = user_check {
        return commercyfy_fail!(err.to_string());
    }

    let user = user_check.unwrap();
    if let None = user {
        return commercyfy_fail!("No matching credentials".to_string());
    }

    let portal_user = user.unwrap();
    let parse_hash = PasswordHash::new(&portal_user.password);
    if let Err(_err) = parse_hash {
        return commercyfy_fail!("There was an error handling your request".to_string());
    }

    let parsed_hash = parse_hash.unwrap();

    let is_password_valid = argon2::Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash);
    if let Err(_err) = is_password_valid {
        return commercyfy_fail!("No matching credentials".to_string());
    }

    let current_time = SystemTime::now();
    let current_time_plus_3h = current_time.checked_add(Duration::from_secs(60 * 60 * 3));
    if let None = current_time_plus_3h {
        return commercyfy_fail!("Internal server error".to_string());
    }

    let exp = current_time_plus_3h.unwrap().duration_since(time::UNIX_EPOCH);
    if let Err(_err) = exp {
        return commercyfy_fail!("Internal server error".to_string());
    } 

    let claims = JWTClaims { email: payload.email, exp: exp.unwrap().as_secs(), roles: portal_user.roles };
    let jwt_secret = std::env::var("JWT_TOKEN_SECRET");
    if let Err(_err) = &jwt_secret {
        return commercyfy_fail!("There was an error logging in.".to_string());
    }

    let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.unwrap().as_bytes()));
    if let Err(err) = token {
        return commercyfy_fail!(err.to_string());
    }

    return commercyfy_success!(SignInToken {jwt: token.unwrap()});
}
