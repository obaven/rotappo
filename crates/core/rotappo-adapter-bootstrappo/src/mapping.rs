use std::collections::HashMap;
use bootstrappo::config::Config;
use bootstrappo::ops::k8s::cache::ClusterCache;
use bootstrappo::ops::reconciler::plan::{Gate, Step};
use validator::Validate;

pub fn driver_specs() -> HashMap<String, (String, Option<String>)> {
    bootstrappo::components::registry::get_all_specs()
        .into_iter()
        .map(|spec| {
            (
                spec.name.to_string(),
                (
                    spec.domain.to_string(),
                    spec.namespace.map(|namespace| namespace.to_string()),
                ),
            )
        })
        .collect()
}

pub fn derive_pod_value(step: &Step, namespace: Option<&str>) -> Option<String> {
    let gate_label = step.gates.iter().find_map(|gate| match gate {
        Gate::DaemonsetReady { namespace, name } => Some(format!("{}/{}", namespace, name)),
        Gate::DeploymentReady { namespace, name } => Some(format!("{}/{}", namespace, name)),
        Gate::StatefulsetReady { namespace, name } => Some(format!("{}/{}", namespace, name)),
        Gate::SecretExists { namespace, name } => Some(format!("{}/{}", namespace, name)),
        _ => None,
    });

    gate_label.or_else(|| namespace.map(|ns| format!("{}/{}", ns, step.id)))
}

pub fn gates_ready(cache: &ClusterCache, step: &Step, config: Option<&Config>) -> bool {
    if step.gates.is_empty() {
        return true;
    }
    for gate in &step.gates {
        match gate {
            Gate::DaemonsetReady { namespace, name } => {
                if !cache.is_daemonset_ready(namespace, name) {
                    return false;
                }
            }
            Gate::DeploymentReady { namespace, name } => {
                if !cache.is_deployment_ready(namespace, name) {
                    return false;
                }
            }
            Gate::StatefulsetReady { namespace, name } => {
                if !cache.is_statefulset_ready(namespace, name) {
                    return false;
                }
            }
            Gate::CrdEstablished { name } => {
                if !cache.is_crd_established(name) {
                    return false;
                }
            }
            Gate::SecretExists { namespace, name } => {
                if !cache.is_secret_ready(namespace, name) {
                    return false;
                }
            }
            Gate::OidcValid => {
                let Some(config) = config else {
                    return false;
                };
                if !config.security.authelia.oidc.enabled {
                    return false;
                }
                if config.security.authelia.validate().is_err() {
                    return false;
                }
            }
        }
    }
    true
}
