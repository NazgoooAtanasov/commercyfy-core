use std::sync::Arc;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2, PasswordHash, PasswordVerifier
};
use actix_web::{Responder, post, web, HttpResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, types::{ToSql, FromSql}, Row};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::models::error::ErrorResponse;

#[derive(Serialize, Deserialize, Debug, Clone, ToSql, FromSql, PartialEq)]
pub enum PortalUsersRoles {
    READER,
    EDITOR,
    ADMIN
}

#[derive(Deserialize, Debug)]
pub struct PortalUser {
    id: uuid::Uuid,
    email: String,
    first_name: String,
    last_name: String,
    password: String,
    roles: Option<Vec<PortalUsersRoles>>
}

impl From<&Row> for PortalUser {
    fn from(row: &Row) -> Self {
        return Self{
            id: row.get("id"),
            email: row.get("email"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            password: row.get("password"),
            roles: row.get("roles")
        }
    }
}

#[post("/user/create")]
pub async fn create_user(app_data: web::Data<Arc<Client>>, data: web::Json<PortalUser>) -> impl Responder {
    let regex = regex::Regex::new(r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$").unwrap();

    if !regex.is_match(data.email.as_str()) {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .json(ErrorResponse{ error_message: String::from("The provided email is not valid.") });
    }

    if data.first_name.len() <= 0 {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .json(ErrorResponse{ error_message: String::from("First name should not be empty.") });
    }

    if data.last_name.len() <= 0 {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .json(ErrorResponse{ error_message: String::from("Last name should not be empty.") });
    }

    if data.password.len() <= 4 {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .json(ErrorResponse{ error_message: String::from("Password should be at least 5 characters.") });
    }

    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(data.password.as_bytes(), &salt);
    if let Err(_) = password_hash {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .json(ErrorResponse{ error_message: String::from("There was an error trying to save the password.") });
    }
    let hash = password_hash.unwrap().to_string();

    let mut roles: Vec<PortalUsersRoles> = vec![];
    if let Some(data_roles) = &data.roles {
        roles = data_roles.to_vec();
    }

    let insert_user_query = app_data.query(" \
        INSERT INTO portal_users (email, first_name, last_name, password, roles) \
        VALUES ($1, $2, $3, $4, $5); \
        ", &[&data.email, &data.first_name, &data.last_name, &hash, &roles]).await;

    if let Err(_) = insert_user_query {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .json(ErrorResponse{
                error_message: String::from("There was an error creating your user.")
            });
    }

    return HttpResponse::build(StatusCode::CREATED)
        .finish()
}


#[derive(Deserialize)]
pub struct Singin {
    email: String,
    password: String
}

#[derive(Serialize, Deserialize)]
struct Token {
    jwt: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JWTClaims {
    pub exp: u64,
    pub email: String,
    pub roles: Vec<PortalUsersRoles>
}

#[post("/signin")]
pub async fn signin(app_data: web::Data<Arc<Client>>, data: web::Json<Singin>) -> impl Responder {
    if data.email.len() == 0 {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .json(ErrorResponse{
                error_message: String::from("Email is required.")
            });
    }

    if data.password.len() == 0 {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .json(ErrorResponse{
                error_message: String::from("Password is required.")
            });
    }

    let portal_user_lookup = app_data.query_one("SELECT * FROM portal_users WHERE email = $1", &[&data.email]).await;

    if let Ok(user) = portal_user_lookup {
        let user = PortalUser::from(&user);
        let parsed_hash = PasswordHash::new(user.password.as_str());

        if let Ok(hash) = parsed_hash {
            let is_valid = Argon2::default().verify_password(data.password.as_bytes(), &hash).is_ok();

            let current_time = std::time::SystemTime::now();
            let current_time_plus_3h = current_time.checked_add(std::time::Duration::from_secs(60 * 60 * 3)).expect("If this ever happens, scream for help.");
            let exp = current_time_plus_3h.duration_since(std::time::UNIX_EPOCH).expect("If this ever happens, scream for help.");

            if is_valid {
                let claims = JWTClaims{
                    exp: exp.as_secs(),
                    email: user.email,
                    roles: user.roles.unwrap_or(vec![])
                };
                let jwt_token_secret = std::env::var("JWT_TOKEN_SECRET").expect("JWT_TOKEN_SECRET MUST BE SET");
                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_token_secret.as_bytes())).unwrap(); // @TODO: handle token creation error

                return HttpResponse::build(StatusCode::OK)
                    .json(Token{
                        jwt: token
                    });
            }

            return HttpResponse::build(StatusCode::BAD_REQUEST)
                .json(ErrorResponse{
                    error_message: String::from("Password is not valid.")
                });
        }

        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .json(ErrorResponse{
                error_message: String::from("Password is not valid.")
            });
    }

    return HttpResponse::build(StatusCode::BAD_REQUEST)
        .json(ErrorResponse{
            error_message: String::from("User with that email does not exist.")
        });
}
