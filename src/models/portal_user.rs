use sqlx::postgres::{PgHasArrayType, PgTypeInfo};

#[derive(serde::Serialize, serde::Deserialize, sqlx::Type, Clone, PartialEq, Eq)]
#[sqlx(type_name = "portaluserroles")]
pub enum PortalUsersRoles {
    READER,
    EDITOR,
    ADMIN,
}

impl PgHasArrayType for PortalUsersRoles {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_portaluserroles")
    }
}

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct PortalUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,

    #[serde(skip_serializing)]
    pub password: String,

    pub roles: Vec<PortalUsersRoles>,
}

#[derive(serde::Serialize)]
pub struct SignInToken {
    pub jwt: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct JWTClaims {
    pub email: String,
    pub exp: u64,
    pub roles: Vec<PortalUsersRoles>,
}
