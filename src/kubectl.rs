extern crate serde;
extern crate serde_json;

use errors::ApplicationError;
use policies::Manifest;
use std::error::Error;
use subprocess::{Exec, Redirection};

pub fn get_network_policies(labels: Option<&str>) -> Result<Manifest, ApplicationError> {
  let mut args = vec!["get", "networkpolicies", "-o", "json"];

  if let Some(labels_value) = labels {
    args.push("-l");
    args.push(labels_value);
  }

  let output = Exec::cmd("kubectl")
    .args(&args)
    .stdout(Redirection::Pipe)
    .capture()
    .unwrap_or_else(|e| panic!("Failed to execute kubectl: {}", e.description()));

  if !output.exit_status.success() {
    return Err(ApplicationError::new(
      "Kubectl failed to retrieve network policies",
    ));
  }

  let policies = serde_json::from_slice(&output.stdout)?;
  return Ok(policies);
}
