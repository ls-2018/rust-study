#![allow(unused)]
use argon2::{
    Argon2, password_hash,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

fn hash_password(password: &str) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, password_hash::Error> {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(password_hash)?;

    // Verify password
    let is_valid = argon2.verify_password(password.as_bytes(), &password_hash).is_ok();

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cp() -> anyhow::Result<(), password_hash::Error> {
        let p = "asd";
        let h = hash_password(p)?;
        println!("{}", h);
        println!("{:?}", verify_password(p, h.as_str()));
        Ok(())
    }
}

fn main() {}
