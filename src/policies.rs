extern crate serde;
extern crate serde_json;

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
#[serde(untagged)]
pub enum Spec {
  #[serde(rename_all = "camelCase")]
  Ingress {
    ingress: Vec<Rule>,
    pod_selector: PodSelector,
    policy_types: Vec<String>,
  },

  #[serde(rename_all = "camelCase")]
  Egress {
    egress: Vec<Rule>,
    pod_selector: PodSelector,
    policy_types: Vec<String>,
  },
}

#[derive(Debug, Deserialize)]
pub struct Item {
  pub spec: Spec,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
  pub api_version: String,
  pub items: Vec<Item>,
  pub kind: String,
}
