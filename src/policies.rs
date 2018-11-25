use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSelector {
  pub match_labels: HashMap<String, String>,
}

impl PodSelector {
  pub fn to_str(&self) -> String {
    self
      .match_labels
      .iter()
      .map(|(k, v)| format!("{}={}", k, v))
      .collect::<Vec<String>>()
      .join(",")
      .to_string()
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RulePodSelector {
  pub pod_selector: PodSelector,
}

#[derive(Debug, Deserialize)]
pub struct Port {
  pub port: u16,
  pub protocol: String,
}

impl Port {
  pub fn to_str(&self) -> String {
    format!("{} ({})", self.port, self.protocol)
  }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PolicyRule {
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
  pub ingress: Vec<PolicyRule>,
  #[serde(default)]
  pub egress: Vec<PolicyRule>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
  pub spec: PolicySpec,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
  pub items: Vec<Item>,
}
