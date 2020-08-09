use serde::{Deserialize, Serialize};

use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, TokenData};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

pub fn make_token(key: &[u8], sub: &str, role: &str) -> Result<String, Error> {
    if key.len() == 0 {
        panic!("Empty key");
    }
    if sub.len() == 0 {
        panic!("Empty sub");
    }    

    let my_claims =
        Claims { sub: sub.to_string().to_owned(), role: role.to_string().to_owned(), exp: 10000000000 };

    let mut header = Header::default();
    header.kid = Some("signing_key".to_owned());
    header.alg = Algorithm::HS512;

    encode(&header, &my_claims, &EncodingKey::from_secret(key))
}

pub fn decode_token(token: String, key: &[u8]) -> Result<TokenData<Claims>, Error> {
    if token.len() == 0 {
        panic!("Empty token");
    }
    if key.len() == 0 {
        panic!("Empty key");
    }

    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key),
        &Validation::new(Algorithm::HS512),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Empty key")]
    fn test_make_token_empty_key() {
        make_token(b"", "", "").unwrap();
    }

    #[test]
    #[should_panic(expected = "Empty sub")]
    fn test_make_token_empty_sub() {
        make_token(b"key", "", "").unwrap();
    }

    #[test]
    #[should_panic(expected = "Empty token")]
    fn test_decode_token_empty_token() {
        decode_token("".to_string(), b"").unwrap();
    }

    #[test]
    #[should_panic(expected = "Empty key")]
    fn test_decode_token_empty_key() {
        decode_token("thisisatoken".to_string(), b"").unwrap();
    }
}
