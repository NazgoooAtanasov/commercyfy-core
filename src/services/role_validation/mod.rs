use crate::models::portal_user::{JWTClaims, PortalUsersRoles};

pub trait RoleService {
    fn validate_role(&self, claims: &JWTClaims, role: PortalUsersRoles) -> Result<(), String>;
    fn validate_admin(&self, claims: &JWTClaims) -> Result<(), String>;
    fn validate_editor(&self, claims: &JWTClaims) -> Result<(), String>;
    fn validate_reader(&self, claims: &JWTClaims) -> Result<(), String>;
    fn validate_any(&self, claims: &JWTClaims, roles: Vec<PortalUsersRoles>) -> Result<(), String>;
}

#[derive(Default)]
pub struct RoleValidation {}

impl RoleService for RoleValidation {
    fn validate_role(&self, claims: &JWTClaims, role: PortalUsersRoles) -> Result<(), String> {
        let checked_role = claims.roles.iter().find(|x| **x == role);

        if let None = checked_role {
            return Err("Not enough permissions".to_string());
        }

        return Ok(());
    }

    fn validate_admin(&self, claims: &JWTClaims) -> Result<(), String> {
        return self.validate_role(claims, PortalUsersRoles::ADMIN);
    }

    fn validate_editor(&self, claims: &JWTClaims) -> Result<(), String> {
        return self.validate_role(claims, PortalUsersRoles::EDITOR);
    }

    fn validate_reader(&self, claims: &JWTClaims) -> Result<(), String> {
        return self.validate_role(claims, PortalUsersRoles::READER);
    }

    fn validate_any(&self, claims: &JWTClaims, roles: Vec<PortalUsersRoles>) -> Result<(), String> {
        let has_role = claims.roles.iter().any(|x| roles.contains(x));

        if has_role {
            return Ok(());
        }

        return Err("Not enough permissions".to_string());
    }
}
