use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};
use tower_sessions::Session;

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
struct UserData {
    authenticated: bool,
    email: Option<String>
}

pub struct User {
    session: Session,
    user_data: UserData
}

impl User {
    const USER_DATA_KEY: &'static str = "user.data";

    pub fn is_authenticated(&self) -> bool {
        return self.user_data.authenticated;
    }

    pub fn get_email(&self) -> Option<String> {
        return self.user_data.email.clone();
    }

    pub async fn set_authenticated(&mut self, email: String) {
        self.user_data.authenticated = true;
        self.user_data.email = Some(email);
        User::update_session(&self.session, &self.user_data).await;
    }

    async fn update_session(session: &Session, data: &UserData) {
        session.insert(User::USER_DATA_KEY, data.clone())
            .await
            .unwrap();
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User where S: Send + Sync, {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(req, state).await?;

        let user_data: UserData = session
            .get(User::USER_DATA_KEY)
            .await
            .unwrap()
            .unwrap_or_default();

        return Ok(Self{
            session,
            user_data
        });
    }
}
