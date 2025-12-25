use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JwtClaims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl JwtClaims {
    pub fn new(sub: Uuid, dur: chrono::Duration) -> Self {
        let now = chrono::Utc::now();

        Self {
            sub,
            iat: now.timestamp(),
            exp: (now + dur).timestamp(),
        }
    }

    pub fn encode(&self, secret: &str) -> Result<String, Error> {
        jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    pub fn decode(token: String, secret: &str) -> Result<Self, Error> {
        jsonwebtoken::decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map(|op| op.claims)
    }
}
