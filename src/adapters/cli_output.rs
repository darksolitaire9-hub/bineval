use crate::ports::OutputPort;
use serde_json::Value;

pub struct CliOutputAdapter;

impl OutputPort for CliOutputAdapter {
    fn write_json(&self, path: &str, data: &Value) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(data)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    fn print_report(&self, report: &str) -> anyhow::Result<()> {
        println!("{}", report);
        Ok(())
    }
}
