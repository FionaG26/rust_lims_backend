use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// Define the structure of the JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // Subject (username)
    pub exp: usize,     // Expiry (Unix timestamp)
}

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Failed to hash password")
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap_or(false)
}

pub fn generate_jwt(username: &str, secret: &str) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() + 3600; // Token expires in 1 hour

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn decode_jwt(token: &str, secret: &str) -> Option<Claims> {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    let validation = Validation {
        leeway: 0,
        validate_exp: true,
        ..Validation::default()
    };
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    );

    match token_data {
        Ok(c) => Some(c.claims),
        Err(_) => None,
    }
}
