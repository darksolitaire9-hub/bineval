use serde::{Deserialize, Serialize};
use serde_json::Value;

const EPSILON: f64 = 1e-6;

fn extract_numbers(text: &str) -> Vec<String> {
    let cleaned = text.replace(',', "");
    let mut numbers = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = cleaned.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let is_neg_start = c == '-'
            && i + 1 < chars.len()
            && chars[i + 1].is_ascii_digit()
            && (i == 0 || !chars[i - 1].is_ascii_digit());
        if c.is_ascii_digit() || c == '.' || is_neg_start {
            current.push(c);
        } else {
            if !current.is_empty() {
                numbers.push(current.clone());
                current.clear();
            }
        }
        i += 1;
    }
    if !current.is_empty() {
        numbers.push(current);
    }
    numbers
        .into_iter()
        .filter_map(|s| {
            let trimmed = s.trim_matches('.');
            if trimmed.parse::<f64>().is_ok() {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn numbers_from_value(val: &serde_json::Value) -> Vec<f64> {
    match val {
        serde_json::Value::String(s) => extract_numbers(s)
            .iter()
            .filter_map(|n| n.parse::<f64>().ok())
            .collect(),
        serde_json::Value::Number(n) => n.as_f64().into_iter().collect(),
        serde_json::Value::Array(arr) => arr.iter().flat_map(numbers_from_value).collect(),
        _ => vec![],
    }
}

fn contains_number(haystack: &[f64], needle: f64) -> bool {
    haystack.iter().any(|h| (h - needle).abs() < EPSILON)
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operator {
    #[default]
    Eq,
    NotEq,
    Exists,
    NotExists,
    Contains,
    #[serde(rename = "numeric_subset")]
    NumericSubset,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub description: Option<String>,
    pub path: String,
    pub operator: Operator,
    pub expected: Option<Value>,
}

impl Policy {
    pub fn evaluate(&self, primitive_data: &Value) -> Result<(), String> {
        let val_opt = if self.path.is_empty() {
            Some(primitive_data)
        } else {
            primitive_data.get(&self.path)
        };

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
            Operator::NumericSubset => {
                let expected = self.expected.as_ref().ok_or_else(|| {
                    "Missing 'expected' value for 'numeric_subset' operator".to_string()
                })?;
                let val = val_opt.ok_or_else(|| format!("Key '{}' does not exist", self.path))?;
                let found = numbers_from_value(val);
                let reference = numbers_from_value(expected);
                for n in &found {
                    if !contains_number(&reference, *n) {
                        return Err(format!(
                            "Hallucinated number found: {} not in reference set",
                            n
                        ));
                    }
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod numeric_subset_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn numeric_subset_passes_when_all_numbers_referenced() {
        let policy = Policy {
            path: "target".to_string(),
            operator: Operator::NumericSubset,
            expected: Some(json!([120, 45.5, 2026])),
            ..Default::default()
        };
        let val = json!({ "target": "Revenue was 120 with margin 45.5 in 2026." });
        assert!(policy.evaluate(&val).is_ok());
    }

    #[test]
    fn numeric_subset_fails_on_hallucinated_number() {
        let policy = Policy {
            path: "target".to_string(),
            operator: Operator::NumericSubset,
            expected: Some(json!([120, 45.5])),
            ..Default::default()
        };
        let val = json!({ "target": "Revenue was 999 last quarter." });
        let result = policy.evaluate(&val);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("999"));
    }

    #[test]
    fn numeric_subset_handles_array_input() {
        let policy = Policy {
            path: "target".to_string(),
            operator: Operator::NumericSubset,
            expected: Some(json!([10, 20, 30])),
            ..Default::default()
        };
        let val = json!({ "target": ["10", "20"] });
        assert!(policy.evaluate(&val).is_ok());
    }

    #[test]
    fn numeric_subset_respects_epsilon_for_floats() {
        let policy = Policy {
            path: "target".to_string(),
            operator: Operator::NumericSubset,
            expected: Some(json!([3.14159265])),
            ..Default::default()
        };
        let val = json!({ "target": "pi is about 3.1415927" });
        assert!(policy.evaluate(&val).is_ok());
    }

    #[test]
    fn numeric_subset_handles_negative_and_comma_formatted_numbers() {
        let policy = Policy {
            path: "target".to_string(),
            operator: Operator::NumericSubset,
            expected: Some(json!([-1234.5, 1000000])),
            ..Default::default()
        };
        let val = json!({ "target": "Net loss was -1,234.50 out of 1,000,000 total budget." });
        assert!(policy.evaluate(&val).is_ok());
    }
}
