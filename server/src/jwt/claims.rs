use chrono::{Duration, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::borrow::Cow;

use crate::error::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JwtClaims<'a> {
  // Time after which the JWT expires
  pub exp: i64,

  // Time before which the JWT must not be accepted for processing
  pub nbf: i64,

  // Time at which the JWT was issued; can be used to determine age of the JWT
  pub iat: i64,

  // Issuer of the JWT
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(borrow)]
  pub iss: Option<Cow<'a, str>>,

  // Subject of the JWT (the user)
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(borrow)]
  pub sub: Option<Cow<'a, str>>,

  // Recipient for which the JWT is intended
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(borrow)]
  pub aud: Option<Cow<'a, str>>,

  // Unique identifier; can be used to prevent the JWT from being replayed
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(borrow)]
  pub jti: Option<Cow<'a, str>>,

  #[serde(flatten)]
  pub extra: HashMap<&'a str, serde_json::Value>
}

impl<'a> JwtClaims<'a> {
  // Create a new claim set.
  pub fn new() -> Self {
    Self { ..Default::default() }
  }
}

#[derive(Debug, Clone)]
pub struct JwtClaimsBuilder<'a> {
  exp: Option<i64>,
  nbf: Option<i64>,
  iat: Option<i64>,
  iss: Option<Cow<'a, str>>,
  sub: Option<Cow<'a, str>>,
  aud: Option<Cow<'a, str>>,
  jti: Option<Cow<'a, str>>,

  extra: HashMap<String, serde_json::Value>
}

impl<'a> Default for JwtClaimsBuilder<'a> {
  fn default() -> Self {
    JwtClaimsBuilder {
      exp: Some(0),
      nbf: None,
      iat: None,
      iss: None,
      sub: None,
      aud: None,
      jti: None,
      extra: HashMap::new()
    }
  }
}

impl<'a> JwtClaimsBuilder<'a> {
  pub fn new() -> Self {
    JwtClaimsBuilder {..Default::default() }
  }

  // Sets the ISS claim
  pub fn issuer<I>(&mut self, value: I) -> &mut Self
    where I: Into<Cow<'a, str>> {
    self.iss = Some(value.into());
    self
  }

  // Sets the SUB claim
  pub fn subject<S>(&mut self, value: S) -> &mut Self
    where S: Into<Cow<'a, str>> {
    self.sub = Some(value.into());
    self
  }

  // Sets the AUD claim
  pub fn audience<A>(&mut self, value: A) -> &mut Self
    where A: Into<Cow<'a, str>> {
    self.aud = Some(value.into());
    self
  }

  // Sets the EXP claim using a duration
  pub fn expires_in(&mut self, expires: Duration) -> &mut Self {
    let exp  = Utc::now() + expires;
    self.exp = Some(exp.timestamp());
    self
  }

  // Sets the EXP claim directly
  pub fn expires<EXP: Into<i64>>(&mut self, expires: EXP) -> &mut Self {
    self.exp = Some(expires.into());
    self
  }

  // Sets the NBF claim
  pub fn not_before<NBF: Into<i64>>(&mut self, not_before: NBF) -> &mut Self {
    self.nbf = Some(not_before.into());
    self
  }

  // Adds an additional claim
  pub fn add_claim<K: Into<Cow<'a, str>>>(&mut self, key: K, value: serde_json::Value) -> &mut Self {
    self.extra.insert(key.into().to_string(), value);
    self
  }

  pub fn build(&self) -> Result<JwtClaims<'a>, HeimdallrError> {
    // iss: None,
    // sub: None,
    // aud: None,
    // jti: None,
    // extra: HashMap::new()
    Ok(JwtClaims {
      exp: match self.exp {
        Some(value) => value,
        None        => return Err(HeimdallrError::JwtError("Expiration must be initialized"))
      },
      nbf: self.nbf.unwrap_or_else(|| Utc::now().timestamp()),
      iat: self.iat.unwrap_or_else(|| Utc::now().timestamp()),
      // iss: self.iss.as_ref(),
      ..Default::default()
    })
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
      .expires_in(Duration::seconds(42))
      .build()?;

    assert_eq!(claims.exp, expected_exp.timestamp());
    Ok(())
  }

  #[test]
  fn test_builder_not_before() -> Result<(), HeimdallrError> {
    let expected_nbf = Utc::now().timestamp();

    let claims = JwtClaimsBuilder::default()
      .not_before(expected_nbf)
      .build()?;

    assert_eq!(claims.nbf, expected_nbf);
    Ok(())
  }
}
