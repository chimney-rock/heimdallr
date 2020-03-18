// use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

mod claims;
pub use claims::*;

#[derive(Debug)]
pub enum JwtType {
  AccessToken,
  RefreshToken
}
