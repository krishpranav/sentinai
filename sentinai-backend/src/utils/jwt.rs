use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
}

pub fn create_token(user_id: Uuid, secret: &str) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::hours(24);

    let claims = Claims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalServerError(format!("Token creation failed: {}", e)))
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::AuthError("Invalid token".into()))?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_verify_token() {
        let user_id = Uuid::new_v4();
        let secret = "supersecret";

        let token = create_token(user_id, secret).expect("Should create token");
        assert!(!token.is_empty());

        let claims = verify_token(&token, secret).expect("Should verify token");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_verify_invalid_signature() {
        let user_id = Uuid::new_v4();
        let token = create_token(user_id, "correct-secret").unwrap();

        let result = verify_token(&token, "wrong-secret");
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::AuthError(msg) => assert_eq!(msg, "Invalid token"),
            _ => panic!("Expected AuthError"),
        }
    }

    #[test]
    fn test_verify_malformed_token() {
        let result = verify_token("not-a-real-jwt", "secret");
        assert!(result.is_err());
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_token_creation_and_verification_proptest(
            secret in "[a-zA-Z0-9_\\-]{10,64}",
            u in any::<u128>()
        ) {
            let user_id = Uuid::from_u128(u);
            let token = create_token(user_id, &secret).expect("Should create token");
            let claims = verify_token(&token, &secret).expect("Should verify token");
            prop_assert_eq!(claims.sub, user_id);
        }
    }
}
