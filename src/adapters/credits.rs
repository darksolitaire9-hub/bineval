use crate::core::credits::Credits;
use std::fs;
use std::path::Path;

pub struct CreditsAdapter;

impl CreditsAdapter {
    pub fn load_project_credits(repo_path: &str) -> anyhow::Result<Credits> {
        let p = Path::new(repo_path).join("credits.json");
        if p.exists() {
            let content = fs::read_to_string(p)?;
            let credits: Credits = serde_json::from_str(&content)?;
            Ok(credits)
        } else {
            Ok(Credits::default())
        }
    }
}
