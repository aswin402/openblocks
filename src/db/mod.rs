pub mod connection;
pub mod migrations;
pub mod components;
pub mod templates;

use crate::error::Result;
use crate::models::component::{Component, NewComponent, UpdateComponent};
use crate::models::template::{Template, NewTemplate};
use crate::models::{Stats, CategoryCount, FrameworkCount};
use rusqlite::Connection;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = connection::open_connection(db_path)?;
        Ok(Self { conn })
    }
    pub fn run_migrations(&mut self) -> Result<()> {
        let migrations = migrations::get_migrations();
        migrations.to_latest(&mut self.conn)
            .map_err(|_e| crate::error::OpenBlocksError::Database(rusqlite::Error::InvalidQuery))?;
        Ok(())
    }

    // Component Delegation
    pub fn insert_component(&self, new: &NewComponent) -> Result<Component> {
        components::insert_component(&self.conn, new)
    }

    pub fn get_component(&self, id: &str) -> Result<Component> {
        components::get_component_by_id(&self.conn, id)
    }

    pub fn update_component(&self, update: &UpdateComponent) -> Result<Component> {
        components::update_component(&self.conn, update)
    }

    pub fn delete_component(&self, id: &str) -> Result<()> {
        components::delete_component(&self.conn, id)
    }

    pub fn list_components(
        &self,
        category: Option<&str>,
        framework: Option<&str>,
        limit: usize,
    ) -> Result<Vec<Component>> {
        components::list_components(&self.conn, category, framework, limit)
    }

    pub fn list_all_components(&self) -> Result<Vec<Component>> {
        components::list_all_components(&self.conn)
    }

    pub fn get_category_counts(&self) -> Result<Vec<CategoryCount>> {
        components::get_category_counts(&self.conn)
    }

    pub fn get_framework_counts(&self) -> Result<Vec<FrameworkCount>> {
        components::get_framework_counts(&self.conn)
    }

    // Template Delegation
    pub fn insert_template(&self, new: &NewTemplate) -> Result<Template> {
        templates::insert_template(&self.conn, new)
    }

    pub fn get_template(&self, id: &str) -> Result<Template> {
        templates::get_template_by_id(&self.conn, id)
    }

    pub fn list_templates(&self) -> Result<Vec<Template>> {
        templates::list_templates(&self.conn)
    }

    pub fn delete_template(&self, id: &str) -> Result<()> {
        templates::delete_template(&self.conn, id)
    }

    // Stats
    pub fn get_stats(&self) -> Result<Stats> {
        let total_components: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM components",
            [],
            |row| row.get(0),
        )?;

        let total_templates: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM templates",
            [],
            |row| row.get(0),
        )?;

        let categories = components::get_category_counts(&self.conn)?;
        let frameworks = components::get_framework_counts(&self.conn)?;

        Ok(Stats {
            total_components,
            total_templates,
            categories,
            frameworks,
        })
    }

    // Seeding
    pub fn seed_from_file(&self, path: &str) -> Result<usize> {
        let content = std::fs::read_to_string(path)?;
        let new_components: Vec<NewComponent> = serde_json::from_str(&content)?;
        let mut count = 0;
        for new in new_components {
            let mut stmt = self.conn.prepare("SELECT 1 FROM components WHERE name = ?1")?;
            let exists = stmt.exists(rusqlite::params![new.name])?;
            if !exists {
                components::insert_component(&self.conn, &new)?;
                count += 1;
            }
        }
        Ok(count)
    }

    // Scaffold
    pub fn scaffold_page(
        &self,
        template_id: &str,
        variables: &serde_json::Value,
    ) -> Result<String> {
        let template = self.get_template(template_id)?;
        
        let mut env = minijinja::Environment::new();

        // Load each component's code
        let mut sections = Vec::new();
        for id in &template.component_ids {
            let component = self.get_component(&id.to_string())?;
            sections.push(component.code);
        }

        // Get or build the base template
        let base = template.layout.get("base_template")
            .and_then(|v| v.as_str())
            .unwrap_or("<!DOCTYPE html>\n<html>\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n  <title>{{ title | default('Scaffolded Page') }}</title>\n</head>\n<body>\n{{ sections }}\n</body>\n</html>");

        env.add_template("page", base)
            .map_err(|e| crate::error::OpenBlocksError::Render(e.to_string()))?;

        let tmpl = env.get_template("page")
            .map_err(|e| crate::error::OpenBlocksError::Render(e.to_string()))?;

        let combined_sections = sections.join("\n\n");

        let mut merged_vars = serde_json::Map::new();
        
        // Load default variables from template
        if let Some(obj) = template.variables.as_object() {
            for (k, v) in obj {
                merged_vars.insert(k.clone(), v.clone());
            }
        }
        
        // Override with user-provided variables
        if let Some(obj) = variables.as_object() {
            for (k, v) in obj {
                merged_vars.insert(k.clone(), v.clone());
            }
        }
        
        // Add default sections
        merged_vars.insert("sections".to_string(), serde_json::Value::String(combined_sections));

        let merged_value = serde_json::Value::Object(merged_vars);
        
        let rendered = tmpl.render(merged_value)
            .map_err(|e| crate::error::OpenBlocksError::Render(e.to_string()))?;

        Ok(rendered)
    }
}
