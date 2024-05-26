#[derive(serde::Deserialize)]
pub struct PortalUserCreate {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub roles: Vec<crate::models::portal_user::PortalUsersRoles>,
}

impl PortalUserCreate {
    pub fn validate(&self) -> Result<(), String> {
        if self.email.is_empty() {
            return Err("'email' field is mandatory".to_string());
        }

        if self.first_name.is_empty() {
            return Err("'first_name' field is mandatory".to_string());
        }

        if self.last_name.is_empty() {
            return Err("'last_name' field is mandatory".to_string());
        }

        if self.password.is_empty() {
            return Err("'password' field is mandatory".to_string());
        }

        if self.password.len() <= 4 {
            return Err("'password' should be longer than 4 symbols".to_string());
        }

        return Ok(());
    }
}

#[derive(serde::Deserialize)]
pub struct PortalUserSignin {
    pub email: String,
    pub password: String,
}

impl PortalUserSignin {
    pub fn validate(&self) -> Result<(), String> {
        if self.email.is_empty() {
            return Err("'email' is mandatory field".to_string());
        }

        if self.password.is_empty() {
            return Err("'password' is mandatory field".to_string());
        }

        return Ok(());
    }
}
