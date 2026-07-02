use crate::ports::CreditsPort;
use crate::core::credits::Credits;
use std::fs;
use std::path::Path;

pub struct CreditsAdapter;

impl CreditsPort for CreditsAdapter {
    fn load_project_credits(&self, repo_path: &str) -> anyhow::Result<Credits> {
        let p = Path::new(repo_path).join("credits.json");
        if p.exists() {
            let content = fs::read_to_string(p)?;
            let credits: Credits = serde_json::from_str(&content)?;
            Ok(credits)
        } else {
            Ok(Credits::default())
        }
    }

    fn load_entity_credits(&self, _entity_name: &str) -> anyhow::Result<Credits> {
        // Stub for entity level credits
        Ok(Credits::default())
    }
}
