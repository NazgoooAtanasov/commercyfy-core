use std::future::{ready, Ready};
use crate::routes::portal_user::JWTClaims;

use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse, http::StatusCode, HttpMessage
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{Algorithm, Validation, decode, DecodingKey};

pub struct Authentication;

impl<S> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S
}

// @TODO: fix the copy paste that has occured here.
impl<S> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let authorization = request.headers().get("Authorization");

        match authorization {
            Some(auth) => {
                let auth_header_value = auth.to_str().unwrap_or("");

                if auth_header_value.eq("") {
                    let (request, _) = request.into_parts();
                    let response = HttpResponse::build(StatusCode::UNAUTHORIZED)
                        .finish();

                    return Box::pin(async {
                        return Ok(ServiceResponse::new(request, response));
                    });
                }

                // @TODO: wtf
                let token = String::from(auth_header_value).split(" ").skip(1).collect::<String>();
                if token.eq("") {
                    let (request, _) = request.into_parts();
                    let response = HttpResponse::build(StatusCode::UNAUTHORIZED)
                        .finish();

                    return Box::pin(async {
                        return Ok(ServiceResponse::new(request, response));
                    });
                }

                let validation = Validation::new(Algorithm::HS256);
                let jwt_token_secret = std::env::var("JWT_TOKEN_SECRET").expect("JWT_TOKEN_SECRET MUST BE SET");
                
                match decode::<JWTClaims>(&token, &DecodingKey::from_secret(jwt_token_secret.as_bytes()), &validation) {
                    Ok(t) => {
                        request.extensions_mut().insert(t.claims.clone());
                    }
                    Err(e) => {
                        let (request, _) = request.into_parts();
                        let response = HttpResponse::build(StatusCode::UNAUTHORIZED)
                            .finish();

                        return Box::pin(async {
                            return Ok(ServiceResponse::new(request, response));
                        });
                    }
                };
            },

            None => {
                let (request, _) = request.into_parts();
                let response = HttpResponse::build(StatusCode::UNAUTHORIZED)
                    .finish();

                return Box::pin(async {
                    return Ok(ServiceResponse::new(request, response));
                });
            }
        }

        let fut = self.service.call(request);
        return Box::pin(async move {
            let res = fut.await?;
            return Ok(res);
        });
    }
}
