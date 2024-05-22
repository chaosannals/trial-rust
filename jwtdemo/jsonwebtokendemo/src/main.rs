use std::time::{ Duration };
use std::ops::Add;
use time::{OffsetDateTime};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors::ErrorKind};
// use std::io::Result;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,         // Optional. Audience
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
}

fn main() -> anyhow::Result<()> {
    let key = b"secret";

    let my_claims = Claims {
        aud: "aud".to_owned(),
        exp: OffsetDateTime::now_utc().add(Duration::from_secs(100)).unix_timestamp() as usize, // 必填， 这个必须是有效时间，不然验证就过期。
        iat: 2,
        iss: "iss".to_owned(),
        nbf: 1,
        sub: "sub".to_owned(),
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key)
    )?;

    println!("token: {:}", token);
    
    
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["aud"]);
    validation.set_required_spec_claims(&["exp", "sub", "aud"]);
    let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            ErrorKind::ExpiredSignature => panic!("过期 {:?}", err),
            _ => panic!("Some other errors {:?}", err),
        },
    };
    println!("decode: {:?}", token_data);
    Ok(())
}
