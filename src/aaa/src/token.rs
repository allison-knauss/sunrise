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
    let my_claims =
        Claims { sub: sub.to_string().to_owned(), role: role.to_string().to_owned(), exp: 10000000000 };

    let mut header = Header::default();
    header.kid = Some("signing_key".to_owned());
    header.alg = Algorithm::HS512;

    encode(&header, &my_claims, &EncodingKey::from_secret(key))
}

pub fn decode_token(token: String, key: &[u8]) -> Result<TokenData<Claims>, Error> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key),
        &Validation::new(Algorithm::HS512),
    )
}
