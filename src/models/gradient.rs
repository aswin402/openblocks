use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Gradient {
    pub id: String,
    pub name: String,
    /// The CSS representation of the gradient (e.g. "linear-gradient(135deg, #153e90 0%, #54e346 100%)")
    pub css: String,
    /// The hex color codes used in the gradient
    pub colors: Vec<String>,
    pub tags: Vec<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize, JsonSchema, Clone)]
pub struct NewGradient {
    pub name: String,
    /// CSS representation (e.g. "linear-gradient(...)")
    pub css: String,
    /// Colors used in the gradient
    pub colors: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl NewGradient {
    pub fn validate(&self) -> Result<(), crate::error::OpenBlocksError> {
        if self.name.trim().is_empty() {
            return Err(crate::error::OpenBlocksError::Validation(
                "Gradient name cannot be empty".into(),
            ));
        }
        if self.css.trim().is_empty() {
            return Err(crate::error::OpenBlocksError::Validation(
                "Gradient CSS string cannot be empty".into(),
            ));
        }
        if self.colors.is_empty() {
            return Err(crate::error::OpenBlocksError::Validation(
                "Gradient must contain at least one color".into(),
            ));
        }
        Ok(())
    }
}
