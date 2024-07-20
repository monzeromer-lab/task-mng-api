use argon2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use jsonwebtoken::{
    decode, encode, errors::Error as JWT_ERROR, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use serde::{Deserialize, Serialize};

use crate::configs::configs;

#[derive(Debug, Clone)]
pub struct Auth<'a> {
    pub argon: Argon2<'a>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthintecationToken {
    pub username: String,
    pub user_id: i32,
}

impl<'a> Auth<'a> {
    pub fn hash_password(&self, password: String) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let pass = b"{password}";
        let hash = self.argon.hash_password(pass, &salt)?;
        Ok(hash.to_string())
    }

    pub fn verify_password(
        &self,
        password: String,
        hashed_password: String,
    ) -> Result<bool, Error> {
        let parsed_hash = PasswordHash::new(&hashed_password)?;
        let pass = b"{password}";
        let result = self.argon.verify_password(pass, &parsed_hash).is_ok();
        Ok(result)
    }

    pub fn make_token(&self, token: AuthintecationToken) -> Result<String, JWT_ERROR> {
        encode(
            &Header::default(),
            &token,
            &EncodingKey::from_secret(configs().secret_key.as_ref()),
        )
    }

    pub fn decode_token(&self, token: &str) -> Result<TokenData<AuthintecationToken>, JWT_ERROR> {
        decode(
            token,
            &DecodingKey::from_secret(configs().secret_key.as_ref()),
            &Validation::default(),
        )
    }
}
