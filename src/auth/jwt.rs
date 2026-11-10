use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::role::Role;

const TOKEN_EXPIRATION_SECONDS: u64 = 60 * 60;

#[derive(Clone)]
pub struct JwtService {
    secret: Arc<[u8]>,
}

#[derive(Clone, Copy, Debug)]
pub struct AuthenticatedUser {
    pub user_id: i32,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: Role,
    exp: u64,
    iat: u64,
}

impl JwtService {
    pub fn new(secret: String) -> Result<Self, &'static str> {
        if secret.len() < 32 {
            return Err("JWT_SECRET harus memiliki minimal 32 byte acak");
        }

        Ok(Self {
            secret: Arc::from(secret.into_bytes()),
        })
    }

    pub fn issue(&self, user_id: i32, role: Role) -> Result<String, jsonwebtoken::errors::Error> {
        let issued_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let claims = Claims {
            sub: user_id.to_string(),
            role,
            iat: issued_at,
            exp: issued_at + TOKEN_EXPIRATION_SECONDS,
        };

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
    }

    pub fn verify(&self, token: &str) -> Result<AuthenticatedUser, ()> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.algorithms = vec![Algorithm::HS256];

        let claims = decode::<Claims>(token, &DecodingKey::from_secret(&self.secret), &validation)
            .map_err(|_| ())?
            .claims;

        let user_id = claims
            .sub
            .parse::<i32>()
            .ok()
            .filter(|user_id| *user_id > 0)
            .ok_or(())?;

        Ok(AuthenticatedUser {
            user_id,
            role: claims.role,
        })
    }

    pub fn expiration_seconds(&self) -> u64 {
        TOKEN_EXPIRATION_SECONDS
    }
}

#[cfg(test)]
mod tests {
    use super::JwtService;
    use crate::auth::role::Role;

    #[test]
    fn issued_token_authenticates_the_user() {
        let jwt = JwtService::new("a sufficiently long test secret value".to_owned()).unwrap();
        let token = jwt.issue(42, Role::Pembeli).unwrap();

        let user = jwt.verify(&token).unwrap();
        assert_eq!(user.user_id, 42);
        assert_eq!(user.role, Role::Pembeli);
    }

    #[test]
    fn modified_token_is_rejected() {
        let jwt = JwtService::new("a sufficiently long test secret value".to_owned()).unwrap();
        let mut token = jwt.issue(42, Role::Admin).unwrap();
        token.push('x');

        assert!(jwt.verify(&token).is_err());
    }

    #[test]
    fn short_secret_is_rejected() {
        assert!(JwtService::new("too short".to_owned()).is_err());
    }
}