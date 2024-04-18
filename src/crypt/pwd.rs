use super::{Error, Result};
use crate::config;
use crate::crypt::{encrypt_into_b64u, EncryptContet};

pub fn encrypt_pwd(enc_content: &EncryptContet) -> Result<String> {
    let key = &config().pwd_key;

    let encrypted = encrypt_into_b64u(key, enc_content)?;

    Ok(format!("#01#{encrypted}"))
}

pub fn validate_pwd(enc_content: &EncryptContet, pwd_ref: &str) -> Result<()> {
    let pwd = encrypt_pwd(enc_content)?;

    if pwd == pwd_ref {
        Ok(())
    } else {
        Err(Error::PwdNotMatching)
    }
}