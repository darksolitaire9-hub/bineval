use crate::core::ports::{LoggerPort, PrimitiveRepositoryPort, SuiteRepositoryPort};
use crate::core::result::{AuditResult, PolicyEvaluation};
use crate::core::BinevalError;

pub struct RunSuiteUseCase<'a> {
    pub primitives: &'a dyn PrimitiveRepositoryPort,
    pub suites: &'a dyn SuiteRepositoryPort,
    pub logger: &'a dyn LoggerPort,
}

impl<'a> RunSuiteUseCase<'a> {
    pub fn run_suite(&self, suite_id: &str) -> Result<AuditResult, BinevalError> {
        let suite = self.suites.get_suite(suite_id)?;
        let primitives = self.primitives.list_primitives()?;

        let mut evaluations = Vec::new();
        let mut all_passed = true;

        for primitive in &primitives {
            for policy in &suite.policies {
                let (passed, message) = match policy.evaluate(&primitive.data) {
                    Ok(()) => (true, None),
                    Err(msg) => (false, Some(msg)),
                };

                if !passed {
                    all_passed = false;
                    self.logger.error(&format!(
                        "Policy '{}' failed for primitive '{}': {}",
                        policy.id,
                        primitive.id,
                        message.as_deref().unwrap_or("Unknown error")
                    ));
                }

                evaluations.push(PolicyEvaluation {
                    suite_id: suite.id.clone(),
                    policy_id: policy.id.clone(),
                    primitive_id: primitive.id.clone(),
                    passed,
                    message,
                });
            }
        }

        Ok(AuditResult {
            passed: all_passed,
            evaluations,
        })
    }
}
