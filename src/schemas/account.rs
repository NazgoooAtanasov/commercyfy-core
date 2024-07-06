#[derive(serde::Deserialize, serde_valid::Validate)]
pub struct CreateAccount {
    #[validate(min_length = 1)]
    pub first_name: String,

    #[validate(min_length = 1)]
    pub last_name: String,

    #[validate(min_length = 1)]
    pub email: String,

    #[validate(min_length = 8)]
    pub password: String,

    pub gender: Option<String>,
    pub birthday: Option<sqlx::types::time::Date>,
}
