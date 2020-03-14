// use heimdallr_api::health::{
//   health_server::{Health, HealthServer},
//   HealthCheckRequest, HealthCheckResponse, ServingStatus
// };

// use tonic::{Request, Response, Status};

// #[derive(Default)]
// pub struct HealthHandler {}

// #[tonic::async_trait]
// impl Health for HealthHandler {
//   async fn check(&self, _request: Request<HealthCheckRequest>) -> Result<Response<HealthCheckResponse>, Status> {
//     let reply = HealthCheckResponse {
//       status: ServingStatus::NotServing.into()
//     };

//     Ok(Response::new(reply))
//   }
// }

// pub fn server() -> HealthServer<HealthHandler> {
//   HealthServer::new(HealthHandler::default())
// }
