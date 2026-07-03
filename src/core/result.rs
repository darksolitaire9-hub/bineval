#[derive(Debug, Clone)]
pub struct PolicyEvaluation {
    pub suite_id: String,
    pub policy_id: String,
    pub primitive_id: String,
    pub passed: bool,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuditResult {
    pub passed: bool,
    pub evaluations: Vec<PolicyEvaluation>,
}
