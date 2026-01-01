use hmac::{Hmac, Mac, digest::InvalidLength};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug)]
pub struct Token {
    pub decrypted: String,
    pub encrypted: String,
}

impl Token {
    pub fn new(secret: &str, session: &str) -> Result<Self, InvalidLength> {
        let mut hasher = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();

        hasher.update(session.as_bytes());

        let enc = hex::encode(hasher.finalize().into_bytes());

        Ok(Self {
            decrypted: session.to_string(),
            encrypted: enc,
        })
    }

    pub fn validate(&self, token: &str) -> bool {
        self.encrypted == token
    }
}
