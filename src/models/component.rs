use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::enums::{Category, Framework};
use crate::error::{OpenBlocksError, Result};

/// A UI component stored in the library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub category: Category,
    pub framework: Framework,
    pub code: String,
    pub dependencies: Vec<String>,
    pub tags: Vec<String>,
    pub preview_html: Option<String>,
    pub version: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a new component (from MCP tool)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NewComponent {
    /// Human-readable component name (e.g., "Glass Navbar")
    pub name: String,
    /// Description of what the component does
    pub description: String,
    /// UI category
    pub category: String,
    /// CSS framework used
    pub framework: String,
    /// The actual HTML/CSS/JS code
    pub code: String,
    /// Searchable tags
    pub tags: Vec<String>,
    /// External dependencies (optional)
    #[serde(default)]
    pub dependencies: Vec<String>,
}

impl NewComponent {
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() || self.name.len() > 200 {
            return Err(OpenBlocksError::Validation(
                "Name must be between 1 and 200 characters".into(),
            ));
        }
        if self.description.trim().is_empty() || self.description.len() > 2000 {
            return Err(OpenBlocksError::Validation(
                "Description must be between 1 and 2000 characters".into(),
            ));
        }
        if self.code.trim().is_empty() {
            return Err(OpenBlocksError::Validation("Code cannot be empty".into()));
        }
        if self.tags.is_empty() {
            return Err(OpenBlocksError::Validation(
                "At least one tag is required".into(),
            ));
        }
        // Validate category and framework parse correctly
        self.category
            .parse::<Category>()
            .map_err(|_| OpenBlocksError::InvalidCategory(self.category.clone()))?;
        self.framework
            .parse::<Framework>()
            .map_err(|_| OpenBlocksError::InvalidFramework(self.framework.clone()))?;
        Ok(())
    }
}

/// Input for updating an existing component (all fields optional except id)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateComponent {
    /// Component ID to update
    pub id: String,
    /// New name (optional)
    pub name: Option<String>,
    /// New description (optional)
    pub description: Option<String>,
    /// New category (optional)
    pub category: Option<String>,
    /// New framework (optional)
    pub framework: Option<String>,
    /// New code (optional)
    pub code: Option<String>,
    /// New tags (optional)
    pub tags: Option<Vec<String>>,
    /// New dependencies (optional)
    pub dependencies: Option<Vec<String>>,
}

impl UpdateComponent {
    pub fn validate(&self) -> Result<()> {
        if let Some(ref name) = self.name
            && (name.trim().is_empty() || name.len() > 200)
        {
            return Err(OpenBlocksError::Validation(
                "Updated name must be between 1 and 200 characters".into(),
            ));
        }
        if let Some(ref description) = self.description
            && (description.trim().is_empty() || description.len() > 2000)
        {
            return Err(OpenBlocksError::Validation(
                "Updated description must be between 1 and 2000 characters".into(),
            ));
        }
        if let Some(ref code) = self.code
            && code.trim().is_empty()
        {
            return Err(OpenBlocksError::Validation(
                "Updated code cannot be empty".into(),
            ));
        }
        if let Some(ref tags) = self.tags
            && tags.is_empty()
        {
            return Err(OpenBlocksError::Validation(
                "Updated tags cannot be empty (must have at least one tag)".into(),
            ));
        }
        if let Some(ref category) = self.category {
            category
                .parse::<Category>()
                .map_err(|_| OpenBlocksError::InvalidCategory(category.clone()))?;
        }
        if let Some(ref framework) = self.framework {
            framework
                .parse::<Framework>()
                .map_err(|_| OpenBlocksError::InvalidFramework(framework.clone()))?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[allow(dead_code)]
pub struct SearchQuery {
    /// Search text (fuzzy matched against name, description, tags)
    pub query: String,
    /// Filter by category (optional)
    pub category: Option<String>,
    /// Filter by framework (optional)
    pub framework: Option<String>,
    /// Filter by tags (optional)
    pub tags: Option<Vec<String>>,
    /// Maximum number of results (default: 10)
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[allow(dead_code)]
fn default_limit() -> usize {
    10
}

/// A search result (component metadata without full code)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SearchResult {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub category: Category,
    pub framework: Framework,
    pub tags: Vec<String>,
    pub version: u32,
    pub score: f64,
}
