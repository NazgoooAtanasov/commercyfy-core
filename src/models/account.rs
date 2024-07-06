pub struct Account {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub gender: Option<String>,
    pub birthday: Option<sqlx::types::time::Date>,
    pub password: String,
}
