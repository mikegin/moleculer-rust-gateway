use temporal_sdk_core::{ClientOptionsBuilder, ClientOptions, Url};
use temporal_client::{Client, WorkflowClientTrait, WorkflowOptions};
use uuid::Uuid;
use std::{
  convert::TryFrom, env, future::Future, net::SocketAddr, path::PathBuf, sync::Arc,
  time::Duration,
};

pub const INTEG_SERVER_TARGET_ENV_VAR: &str = "TEMPORAL_SERVICE_ADDRESS";

fn get_integ_server_options() -> ClientOptions {
  let temporal_server_address = match env::var(INTEG_SERVER_TARGET_ENV_VAR) {
      Ok(addr) => addr,
      Err(_) => "http://localhost:7233".to_owned(),
  };
  let url = Url::try_from(&*temporal_server_address).unwrap();
  ClientOptionsBuilder::default()
      .identity("integ_tester".to_string())
      .target_url(url)
      .client_name("temporal-core".to_string())
      .client_version("0.1.0".to_string())
      .build()
      .unwrap()
}


#[tokio::test]
async fn execute_workflow() {
  let client = Arc::new(
    get_integ_server_options()
        .connect("default", None, None)
        .await
        .expect("Must connect"),
  );

  let wf_id = format!("{}{}", "wf", Uuid::new_v4().to_string().as_str().to_owned());

  client.get_client().start_workflow(vec![], (&"pipelines-processor").to_string(), wf_id, (&"processRealtimeEvent").to_string(), None, WorkflowOptions::default()).await.unwrap();
}