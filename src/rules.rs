use policies::{Manifest, PolicyRule, Port, RulePodSelector};

#[derive(Debug)]
pub struct Rule {
  selector: String,
  origin: String,
  destination: String,
}

fn ports_to_str(ports: &Vec<Port>) -> String {
  ports
    .iter()
    .map(|port| port.to_str())
    .collect::<Vec<String>>()
    .join(", ")
}

fn selectors_to_str(selectors: &Vec<RulePodSelector>) -> String {
  selectors
    .iter()
    .map(|item| item.pod_selector.to_str())
    .collect::<Vec<String>>()
    .join(", ")
}

impl Rule {
  fn from_policy_rule(policy_rule: &PolicyRule, selector: String) -> Rule {
    match policy_rule {
      PolicyRule::Ingress { ports, from } => Rule {
        selector,
        origin: selectors_to_str(&from),
        destination: ports_to_str(&ports),
      },
      PolicyRule::Egress { ports, to } => Rule {
        selector,
        origin: ports_to_str(&ports),
        destination: selectors_to_str(&to),
      },
    }
  }
}

impl Manifest {
  pub fn into_rules(self) -> Vec<Rule> {
    self
      .items
      .into_iter()
      .map(|item| item.spec)
      .fold(vec![], |mut output, spec| {
        output.append(
          &mut spec
            .ingress
            .iter()
            .map(|rule| Rule::from_policy_rule(rule, spec.pod_selector.to_str()))
            .collect(),
        );
        return output;
      })
  }
}
