fn main() -> Result<(), Box<dyn std::error::Error>> {
  let files = &[
    "protos/heath_check.proto",
    "protos/auth.proto",
  ];
  let dirs = &["protos"];

  tonic_build::configure()
    .build_server(true)
    .build_client(true)
    .compile(files, dirs)?;

  for file in files {
    println!("cargo:rerun-if-changed={}", file);
  }

  Ok(())
}
