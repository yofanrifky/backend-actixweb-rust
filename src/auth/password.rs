use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use subtle::ConstantTimeEq;

pub struct PasswordVerification {
    pub valid: bool,
    pub upgraded_hash: Option<String>,
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

pub fn verify_and_upgrade(
    password: &str,
    stored_password: &str,
) -> Result<PasswordVerification, argon2::password_hash::Error> {
    if stored_password.starts_with("$argon2") {
        let valid = PasswordHash::new(stored_password)
            .ok()
            .is_some_and(|parsed| {
                Argon2::default()
                    .verify_password(password.as_bytes(), &parsed)
                    .is_ok()
            });

        return Ok(PasswordVerification {
            valid,
            upgraded_hash: None,
        });
    }

    let valid = bool::from(password.as_bytes().ct_eq(stored_password.as_bytes()));
    let upgraded_hash = valid.then(|| hash_password(password)).transpose()?;

    Ok(PasswordVerification {
        valid,
        upgraded_hash,
    })
}

#[cfg(test)]
mod tests {
    use super::{hash_password, verify_and_upgrade};

    #[test]
    fn legacy_password_is_verified_and_upgraded() {
        let result = verify_and_upgrade("password-lama", "password-lama").unwrap();
        let upgraded_hash = result.upgraded_hash.unwrap();

        assert!(result.valid);
        assert!(upgraded_hash.starts_with("$argon2id$"));
        let verified_hash = verify_and_upgrade("password-lama", &upgraded_hash).unwrap();
        assert!(verified_hash.valid);
        assert!(verified_hash.upgraded_hash.is_none());
    }

    #[test]
    fn incorrect_password_is_rejected() {
        let hash = hash_password("password-benar").unwrap();
        let result = verify_and_upgrade("password-salah", &hash).unwrap();

        assert!(!result.valid);
        assert!(result.upgraded_hash.is_none());
    }
}
