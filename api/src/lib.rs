// pub mod health {
//   tonic::include_proto!("grpc.health.v1");

//   pub use health_check_response::ServingStatus;
//   impl From<i32> for ServingStatus {
//     fn from(item: i32) -> Self {
//       match item {
//         1 => ServingStatus::Serving,
//         2 => ServingStatus::NotServing,
//         _ => ServingStatus::Unknown
//       }
//     }
//   }
// }

pub mod auth {
  tonic::include_proto!("heimdallr.auth");
}

/// Converts a Rust Duration to a Protobuf Duration.
/// Taken from https://github.com/linkerd/linkerd2-proxy-api
pub fn convert_duration(duration: std::time::Duration) -> prost_types::Duration {
  let seconds = if duration.as_secs() > std::i64::MAX as u64 {
    std::i64::MAX
  } else {
    duration.as_secs() as i64
  };

  let nanos = if duration.subsec_nanos() > std::i32::MAX as u32 {
    std::i32::MAX
  } else {
    duration.subsec_nanos() as i32
  };

  prost_types::Duration {
    seconds,
    nanos,
  }
}
