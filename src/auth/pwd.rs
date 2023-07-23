use argon2::{
    password_hash::{
        rand_core::OsRng, Error as HashError, PasswordHash, PasswordHashString, PasswordHasher,
        PasswordVerifier, SaltString,
    },
    Argon2,
};

/// Generate the hash for a new password.
pub fn new_password(password: &str) -> anyhow::Result<String> {
    new_pwd(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}

/// Verify that a password is correct
pub fn verify_password(password: &str, hash: &str) -> anyhow::Result<bool> {
    let hash = PasswordHash::new(hash).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    Ok(verify_pwd(&hash, password.as_bytes()))
}

fn new_pwd(pwd: &[u8]) -> Result<PasswordHashString, HashError> {
    let salt = gen_salt();
    Ok(gen_hash(pwd, &salt)?.serialize())
}

fn verify_pwd(hash: &PasswordHash<'_>, pwd: &[u8]) -> bool {
    Argon2::default().verify_password(pwd, hash).is_ok()
}

fn gen_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

fn gen_hash<'a>(pwd: &[u8], salt: &'a SaltString) -> Result<PasswordHash<'a>, HashError> {
    Argon2::default().hash_password(pwd, salt)
}
