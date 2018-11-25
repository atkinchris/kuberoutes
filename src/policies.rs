use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSelector {
  pub match_labels: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RulePodSelector {
  pod_selector: PodSelector,
}

#[derive(Debug, Deserialize)]
pub struct Port {
  pub port: u16,
  pub protocol: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Rule {
  Ingress {
    from: Vec<RulePodSelector>,
    ports: Vec<Port>,
  },
  Egress {
    to: Vec<RulePodSelector>,
    ports: Vec<Port>,
  },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicySpec {
  pub pod_selector: PodSelector,
  pub policy_types: Vec<String>,
  #[serde(default)]
  pub ingress: Vec<Rule>,
  #[serde(default)]
  pub egress: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
  pub creation_timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
  pub spec: PolicySpec,
  pub metadata: MetaData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
  pub api_version: String,
  pub items: Vec<Item>,
  pub kind: String,
}

#[derive(Debug)]
pub struct Connection {}

impl Manifest {
  fn find_latest_policy_of_type(&self, policy_type: &str) -> Option<&PolicySpec> {
    self
      .items
      .iter()
      .filter(|item| item.spec.policy_types.contains(&policy_type.to_owned()))
      .max_by_key(|item| item.metadata.creation_timestamp)
      .map(|item| &item.spec)
  }

  pub fn into_connections(self) -> Vec<Connection> {
    let connections = Vec::new();

    let active_ingress_policy = self.find_latest_policy_of_type("Ingress");
    let active_igress_policy = self.find_latest_policy_of_type("Egress");

    return connections;
  }
}
