use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Primitive {
    pub id: String,
    pub data: Value,
}
