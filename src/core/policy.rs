use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operator {
    Eq,
    NotEq,
    Exists,
    NotExists,
    Contains,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub description: Option<String>,
    pub path: String,
    pub operator: Operator,
    pub expected: Option<Value>,
}

impl Policy {
    pub fn evaluate(&self, primitive_data: &Value) -> Result<(), String> {
        let val_opt = primitive_data.get(&self.path);

        match self.operator {
            Operator::Exists => {
                if val_opt.is_some() && !val_opt.unwrap().is_null() {
                    Ok(())
                } else {
                    Err(format!("Key '{}' does not exist or is null", self.path))
                }
            }
            Operator::NotExists => {
                if val_opt.is_none() || val_opt.unwrap().is_null() {
                    Ok(())
                } else {
                    Err(format!("Key '{}' exists", self.path))
                }
            }
            Operator::Eq => {
                let expected = self
                    .expected
                    .as_ref()
                    .ok_or_else(|| "Missing 'expected' value for 'eq' operator".to_string())?;
                let val = val_opt.ok_or_else(|| format!("Key '{}' does not exist", self.path))?;
                if val == expected {
                    Ok(())
                } else {
                    Err(format!("Expected '{}', got '{}'", expected, val))
                }
            }
            Operator::NotEq => {
                let expected = self
                    .expected
                    .as_ref()
                    .ok_or_else(|| "Missing 'expected' value for 'noteq' operator".to_string())?;
                let val = val_opt.ok_or_else(|| format!("Key '{}' does not exist", self.path))?;
                if val != expected {
                    Ok(())
                } else {
                    Err(format!("Value is equal to '{}'", expected))
                }
            }
            Operator::Contains => {
                let expected = self.expected.as_ref().ok_or_else(|| {
                    "Missing 'expected' value for 'contains' operator".to_string()
                })?;
                let val = val_opt.ok_or_else(|| format!("Key '{}' does not exist", self.path))?;

                if let Some(arr) = val.as_array() {
                    if arr.contains(expected) {
                        Ok(())
                    } else {
                        Err(format!("Array does not contain '{}'", expected))
                    }
                } else if let Some(s) = val.as_str() {
                    if let Some(exp_str) = expected.as_str() {
                        if s.contains(exp_str) {
                            Ok(())
                        } else {
                            Err(format!("String does not contain '{}'", exp_str))
                        }
                    } else {
                        Err("Expected value for 'contains' on string must be a string".to_string())
                    }
                } else {
                    Err(format!(
                        "Value at '{}' is not an array or string",
                        self.path
                    ))
                }
            }
        }
    }
}
