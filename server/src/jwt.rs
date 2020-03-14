// use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use chrono::{DateTime, Duration, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::borrow::Cow;
use uuid::Uuid;

use crate::error::*;

#[derive(Debug)]
pub enum JwtType {
  AccessToken,
  RefreshToken
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JwtClaims<'a> {
  // Issuer of the JWT
  #[serde(skip_serializing_if = "Option::is_none")]
  pub iss: Option<Cow<'a, str>>,

  // Subject of the JWT (the user)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sub: Option<Cow<'a, str>>,

  // Recipient for which the JWT is intended
  #[serde(skip_serializing_if = "Option::is_none")]
  pub aud: Option<Cow<'a, str>>,

  // Unique identifier; can be used to prevent the JWT from being replayed
  #[serde(skip_serializing_if = "Option::is_none")]
  pub jti: Option<Uuid>,

  // Time after which the JWT expires
  pub exp: i64,

  // Time before which the JWT must not be accepted for processing
  pub nbf: i64,

  // Time at which the JWT was issued; can be used to determine age of the JWT
  pub iat: i64,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub nonce: Option<Cow<'a, str>>,

  #[serde(flatten)]
  pub extra: HashMap<Cow<'a, str>, serde_json::Value>
}

impl<'a> JwtClaims<'a> {
  // Create a new claim set.
  pub fn new() -> Self {
    Self { ..Default::default() }
  }

  pub fn build() -> JwtClaimsBuilder<'a> {
    JwtClaimsBuilder::new()
  }
}

#[derive(Debug, Default, Clone)]
pub struct JwtClaimsBuilder<'a> {
  claims: JwtClaims<'a>
}

impl<'a> JwtClaimsBuilder<'a> {
  pub fn new() -> Self {
    JwtClaimsBuilder { claims: JwtClaims::new() }
  }

  // Sets the ISS claim
  pub fn issuer<I: Into<Cow<'a, str>>>(mut self, value: I) -> Self {
    self.claims.iss = Some(value.into());
    self
  }

  // Sets the SUB claim
  pub fn subject<S: Into<Cow<'a, str>>>(mut self, value: S) -> Self {
    self.claims.sub = Some(value.into());
    self
  }

  // Sets the AUD claim
  pub fn audience<A: Into<Cow<'a, str>>>(mut self, value: A) -> Self {
    self.claims.aud = Some(value.into());
    self
  }

  // Sets the EXP claim
  pub fn expires(mut self, expires: Duration) -> Self {
    let exp = Utc::now() + expires;
    self.claims.exp = exp.timestamp();
    self
  }

  // Sets the NBF claim
  pub fn not_before(mut self, not_before: DateTime<Utc>) -> Self {
    self.claims.nbf = not_before.timestamp();
    self
  }

  // Adds an additional claim
  pub fn add_claim<K: Into<Cow<'a, str>>>(mut self, key: K, value: serde_json::Value) -> Self {
    self.claims.extra.insert(key.into(), value);
    self
  }

  pub fn finish(self) -> Result<JwtClaims<'a>, HeimdallrError> {
    Ok(self.claims)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  #[test]
  fn test_builder_issuer_with_str_slice() {
    JwtClaimsBuilder::default().issuer("Super Kawaii");
  }

  #[test]
  fn test_builder_issuer_with_owned_string() {
    JwtClaimsBuilder::default().issuer(String::from("Super Kawaii"));
  }

  #[test]
  fn test_builder_subject_with_str_slice() {
    JwtClaimsBuilder::default().subject("heimdallr");
  }

  #[test]
  fn test_builder_subject_with_owned_string() {
    JwtClaimsBuilder::default().subject(String::from("heimdallr"));
  }

  #[test]
  fn test_builder_audience_with_str_slice() {
    JwtClaimsBuilder::default().audience("takara@doge.com");
  }

  #[test]
  fn test_builder_audience_with_owned_string() {
    JwtClaimsBuilder::default().audience(String::from("takara@doge.com"));
  }

  #[test]
  fn test_builder_expires() -> Result<(), HeimdallrError> {
    let expected_exp = Utc::now() + Duration::seconds(42);

    let claims = JwtClaimsBuilder::default()
      .expires(Duration::seconds(42))
      .finish()?;

    assert_eq!(claims.exp, expected_exp.timestamp());
    Ok(())
  }

  #[test]
  fn test_builder_not_before() -> Result<(), HeimdallrError> {
    let expected_nbf = Utc::now();

    let claims = JwtClaimsBuilder::default()
      .not_before(expected_nbf)
      .finish()?;

    assert_eq!(claims.nbf, expected_nbf.timestamp());
    Ok(())
  }
}
