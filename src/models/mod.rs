pub mod component;
pub mod enums;
pub mod gradient;
pub mod palette;
pub mod template;

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Stats {
    pub total_components: i64,
    pub total_templates: i64,
    pub categories: Vec<CategoryCount>,
    pub frameworks: Vec<FrameworkCount>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CategoryCount {
    pub category: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct FrameworkCount {
    pub framework: String,
    pub count: i64,
}
