use policies::{Manifest, PolicyRule, Port, RulePodSelector};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rule {
  IngressAllow {
    selector: String,
    from: String,
    to: String,
  },
  EgressAllow {
    selector: String,
    from: String,
    to: String,
  },
  IngressDeny {
    selector: String,
  },
  EgressDeny {
    selector: String,
  },
}

fn ports_to_str(ports: &Vec<Port>) -> String {
  if ports.is_empty() {
    return format!("<any> (traffic allowed to all ports)");
  }

  ports
    .iter()
    .map(|port| port.to_str())
    .collect::<Vec<String>>()
    .join(", ")
}

fn selectors_to_str(selectors: &Vec<RulePodSelector>) -> String {
  if selectors.is_empty() {
    return format!("<any> (traffic not restricted by source)");
  }

  selectors
    .iter()
    .map(|item| item.pod_selector.to_str())
    .collect::<Vec<String>>()
    .join(", ")
}

impl Rule {
  fn from_policy_rule(policy_rule: &PolicyRule, selector: &String) -> Rule {
    let selector = selector.to_owned();

    match policy_rule {
      PolicyRule::Ingress { ports, from } => Rule::IngressAllow {
        selector,
        from: selectors_to_str(&from),
        to: ports_to_str(&ports),
      },
      PolicyRule::Egress { ports, to } => Rule::EgressAllow {
        selector,
        from: ports_to_str(&ports),
        to: selectors_to_str(&to),
      },
    }
  }

  fn create_ingress_deny(selector: &String) -> Rule {
    Rule::IngressDeny {
      selector: selector.to_owned(),
    }
  }

  fn create_egress_deny(selector: &String) -> Rule {
    Rule::EgressDeny {
      selector: selector.to_owned(),
    }
  }
}

impl Manifest {
  pub fn into_rules(self) -> Vec<Rule> {
    let mut rules =
      self
        .items
        .into_iter()
        .map(|item| item.spec)
        .fold(vec![], |mut output, spec| {
          let selector = spec.pod_selector.to_str();

          if spec.ingress.is_empty() {
            output.push(Rule::create_ingress_deny(&selector));
          };

          output.append(
            &mut spec
              .ingress
              .iter()
              .map(|rule| Rule::from_policy_rule(rule, &selector))
              .collect(),
          );

          if spec.egress.is_empty() {
            output.push(Rule::create_egress_deny(&selector));
          };

          output.append(
            &mut spec
              .egress
              .iter()
              .map(|rule| Rule::from_policy_rule(rule, &selector))
              .collect(),
          );

          return output;
        });

    rules.sort_unstable();
    rules.dedup();
    return rules;
  }
}
