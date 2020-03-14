use heimdallr_api::auth::{
  login_server::{Login, LoginServer},
  LoginRequest, LoginResponse
};
use crate::db::Database;

use tonic::{Request, Response, Status};
use std::sync::Arc;

// #[derive(Default)]
pub struct AuthHandler {
  db: Arc<Database>
}

impl AuthHandler {

  pub fn new(db: Database) -> Self {
    Self { db: Arc::new(db) }
  }

  pub fn service(self) -> LoginServer<Self> {
    LoginServer::new(self)
  }

  pub fn database(&self) -> Arc<Database> {
    self.db.clone()
  }
}

#[tonic::async_trait]
impl Login for AuthHandler {
  async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<String>, tonic::Status> {
    Ok(Response::new("pong".to_string()))
  }

  async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
    println!("Login = {:?}", request);

    Ok(Response::new(LoginResponse::default()))
  }
}
