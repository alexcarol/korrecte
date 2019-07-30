use crate::linters::{Lint, LintSpec, Group};

use kube::api::Object;
use k8s_openapi::api::core::v1::{ServiceSpec, ServiceStatus};
use serde::Deserialize;
use crate::reporting::{Reporter, Finding};
use std::collections::{HashMap, BTreeMap};
use crate::kube::objects::ObjectRepository;

/// **What it does:** Checks that services are well defined and has some matching
/// object (defined by the service selector).
///
/// **Why is this bad?** A service without any matching pod is usually a symptom of a
/// bad configuration
///
/// **Known problems:** Sending data to that service may provoke failures
///
/// **References**
pub(crate) struct ServiceWithoutMatchingLabels<R: Reporter> {
    reporter: R,
    object_repository: ObjectRepository,
}

impl<R: Reporter> ServiceWithoutMatchingLabels<R> {
    pub fn new(reporter: R, object_repository: ObjectRepository) -> Self {
        ServiceWithoutMatchingLabels {
            reporter,
            object_repository,
        }
    }
}

impl<R: Reporter> Lint for ServiceWithoutMatchingLabels<R> {
    fn spec(&self) -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "service_without_matching_labels".to_string(),
        }
    }

    fn service(&self, service: &Object<ServiceSpec, ServiceStatus>) {
        let selectors: BTreeMap<String, String> = service.spec.selector.clone().unwrap_or_default();

        let any_matching_pod = self.object_repository.pods()
            .iter()
            .any(|pod| {
                let pod_labels = &pod.metadata.labels;

                selectors.iter()
                    .all(|(k, v)| {
                        pod_labels.get(k)
                            .map(|pod_value| pod_value == v)
                            .unwrap_or(false)
                    })
            });

        if !any_matching_pod {
            let finding= Finding::new(self.spec().clone(), service.metadata.clone());
            self.reporter.report(finding);
        }
    }
}