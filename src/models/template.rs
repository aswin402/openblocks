use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub layout: serde_json::Value,    // Layout definition
    pub component_ids: Vec<Uuid>,     // Ordered component references
    pub variables: serde_json::Value, // Customizable variables
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NewTemplate {
    pub name: String,
    pub description: String,
    pub component_ids: Vec<String>,
    pub layout: serde_json::Value,
    #[serde(default)]
    pub variables: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[allow(dead_code)]
pub struct ScaffoldRequest {
    /// Template ID to scaffold from
    pub template_id: String,
    /// Variable overrides (brand colors, text, etc.)
    #[serde(default)]
    pub variables: serde_json::Value,
}
