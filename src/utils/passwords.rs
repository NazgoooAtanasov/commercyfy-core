use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

pub fn hash_password(pwd: &str) -> Result<String, String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(pwd.as_bytes(), &salt);
    if let Err(err) = password_hash {
        return Err(err.to_string());
    }
    return Ok(password_hash.unwrap().to_string());
}
