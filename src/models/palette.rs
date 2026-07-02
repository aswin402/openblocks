use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Palette {
    pub id: String,
    pub name: String,
    /// JSON array of exactly 4 hex color strings (e.g. ["#222831", "#393e46", "#00adb5", "#eeeeee"])
    pub colors: Vec<String>,
    pub tags: Vec<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize, JsonSchema, Clone)]
pub struct NewPalette {
    pub name: String,
    /// Exactly 4 hex color strings (e.g. ["#1a1a1a", "#2b2b2b", "#3c3c3c", "#4d4d4d"])
    pub colors: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl NewPalette {
    pub fn validate(&self) -> Result<(), crate::error::OpenBlocksError> {
        if self.name.trim().is_empty() {
            return Err(crate::error::OpenBlocksError::Validation(
                "Palette name cannot be empty".into(),
            ));
        }
        if self.colors.len() != 4 {
            return Err(crate::error::OpenBlocksError::Validation(
                "Palette must contain exactly 4 colors".into(),
            ));
        }
        for color in &self.colors {
            if !color.starts_with('#') || (color.len() != 7 && color.len() != 4) {
                return Err(crate::error::OpenBlocksError::Validation(format!(
                    "Invalid hex color format: {}",
                    color
                )));
            }
        }
        Ok(())
    }
}
