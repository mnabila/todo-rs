use bcrypt::BcryptError;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub token: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn hash_password(password: &str) -> Result<String, BcryptError> {
        bcrypt::hash(password, 6)
    }

    pub fn verify_password(&self, password: String) -> bool {
        bcrypt::verify(password, self.password.as_str()).is_ok()
    }
}
