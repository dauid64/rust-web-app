

use std::fmt::Display;
use std::str::FromStr;

use axum::body::HttpBody;

use crate::config;
use crate::crypt::{encrypt_into_b64u, EncryptContet, Error, Result};
use crate::utils::{b64_decode, b64u_encode, now_utc, now_utc_plus_sec_str, parse_utc};

#[derive(Debug)]
pub struct Token {
    pub ident: String,
    pub exp: String,
    pub sign_b64u: String,
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(token_str: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let splits: Vec<&str> = token_str.split(".").collect();
        if splits.len() != 3 {
            return Err(Error::TokenInvalidFormat);
        }
        let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);

        Ok(Self {
            ident: b64_decode(ident_b64u)
                .map_err(|_| Error::TokenCannotDecodeIdent)?,
            
            exp: b64_decode(exp_b64u).map_err(|_| Error::TokenCannotDecodeExp)?,

            sign_b64u: sign_b64u.to_string(),
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            b64u_encode(&self.ident),
            b64u_encode(&self.exp),
            self.sign_b64u
        )
    }
}

pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
    let config = &config();
    _generate_token(user, config.token_duration_sec, salt, &config.token_key)
}

pub fn validate_web_token(origin_token: &Token, salt: &str) -> Result<()> {
    let config = &config();
    _validate_token_sign_and_exp(origin_token, salt, &config.token_key)?;

    Ok(())
}

fn _generate_token(
    ident: &str,
    duration_sec: f64,
    salt: &str,
    key: &[u8],
) -> Result<Token> {
    let ident = ident.to_string();
    let exp = now_utc_plus_sec_str(duration_sec);

    let sign_b64u = _token_sing_into_b64u(&ident, &exp, salt, key)?;
    
    Ok(Token {
        ident,
        exp,
        sign_b64u
    })
}

fn _validate_token_sign_and_exp(
    origin_token: &Token,
    salt: &str,
    key: &[u8],
) -> Result<()> {
    let new_sign_b64u = 
        _token_sing_into_b64u(&origin_token.ident, &origin_token.exp, salt, key)?;
    
    if new_sign_b64u != origin_token.sign_b64u {
        return Err(Error::TokenSignatureNotMatching);
    }

    let origin_exp =
        parse_utc(&origin_token.exp).map_err(|_| Error::TokenExpNotIso)?;
    let now = now_utc();

    if origin_exp < now {
        return Err(Error::TokenExpired);
    }
    
    Ok(())
}

fn _token_sing_into_b64u(
    ident: &str,
    exp: &str,
    salt: &str,
    key: &[u8]
) -> Result<String> {
    let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));
    let signature = encrypt_into_b64u(
        key, 
        &EncryptContet {
            content,
            salt: salt.to_string()
        },
    )?;

    Ok(signature)
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_token_display_ok() -> Result<()> {
        let fx_token_str = 
            "ZngtaWRlbnQtMDE.MjAyMy0wNS0xN1QxNTozMDowMFo.some-sing-b64u-encoded";
        let fx_token = Token {
            ident: "fx-ident-01".to_string(),
            exp: "2023-05-17T15:30:00Z".to_string(),
            sign_b64u: "some-sing-b64u-encoded".to_string(),
        };

        assert_eq!(fx_token.to_string(), fx_token_str);

        Ok(())
    }

    #[test]
    fn test_token_from_str_ok() -> Result<()> {
        let fx_token_str = 
            "ZngtaWRlbnQtMDE.MjAyMy0wNS0xN1QxNTozMDowMFo.some-sing-b64u-encoded";
        let fx_token = Token {
            ident: "fx-ident-01".to_string(),
            exp: "2023-05-17T15:30:00Z".to_string(),
            sign_b64u: "some-sing-b64u-encoded".to_string(),
        };

        let token: Token = fx_token_str.parse()?;

        assert_eq!(format!("{token:?}"), format!("{fx_token:?}"));

        Ok(())
    }

    #[test]
    fn test_validate_web_token_ok() -> Result<()> {
        let fx_user = "user_one";
        let fx_salt = "pepper";
        let fx_duration_sec = 0.2; // 20ms
        let token_key = &config().token_key;
        let fx_token = 
            _generate_token(fx_user, fx_duration_sec, fx_salt, token_key)?;
        thread::sleep(Duration::from_millis(10));
        let res = validate_web_token(&fx_token, fx_salt);

        res?;

        Ok(())
    }

    #[test]
    fn test_validate_web_token_err_expired() -> Result<()> {
        let fx_user = "user_one";
        let fx_salt = "pepper";
        let fx_duration_sec = 0.01; // 10ms
        let token_key = &config().token_key;
        let fx_token = 
            _generate_token(fx_user, fx_duration_sec, fx_salt, token_key)?;
        thread::sleep(Duration::from_millis(10));
        let res = validate_web_token(&fx_token, fx_salt);

        assert!(
            matches!(res, Err(Error::TokenExpired)),
            "Should have matched `Err(Error::TokenExpired)` but was `{res:?}`"
        );

        Ok(())
    }
}