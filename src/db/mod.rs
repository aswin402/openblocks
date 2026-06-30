pub mod connection;
pub mod migrations;
pub mod components;
pub mod templates;
pub mod palettes;
pub mod gradients;

use crate::error::Result;
use crate::models::component::{Component, NewComponent, UpdateComponent};
use crate::models::template::{Template, NewTemplate};
use crate::models::palette::{Palette, NewPalette};
use crate::models::gradient::{Gradient, NewGradient};
use crate::models::{Stats, CategoryCount, FrameworkCount};
use rusqlite::{Connection, params};

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

    pub fn get_stats(&self) -> Result<Stats> {
        let total_components: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM components",
            [],
            |row| row.get(0)
        )?;
        let total_templates: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM templates",
            [],
            |row| row.get(0)
        )?;
        
        let categories = self.get_category_counts()?;
        let frameworks = self.get_framework_counts()?;
        
        Ok(Stats {
            total_components,
            total_templates,
            categories,
            frameworks,
        })
    }

    // Template Delegation
    pub fn insert_template(&self, new: &NewTemplate) -> Result<Template> {
        templates::insert_template(&self.conn, new)
    }

    pub fn get_template(&self, id: &str) -> Result<Template> {
        templates::get_template_by_id(&self.conn, id)
    }

    pub fn delete_template(&self, id: &str) -> Result<()> {
        templates::delete_template(&self.conn, id)
    }

    pub fn list_templates(&self) -> Result<Vec<Template>> {
        templates::list_templates(&self.conn)
    }

    pub fn seed_from_file(&self, path: &str) -> Result<usize> {
        // Read file
        let content = std::fs::read_to_string(path)?;
        let new_components: Vec<NewComponent> = serde_json::from_str(&content)?;
        
        let mut count = 0;
        for new in new_components {
            // Check if name already exists to avoid duplicates during repeated seeding
            let exists: i64 = self.conn.query_row(
                "SELECT COUNT(*) FROM components WHERE name = ?1",
                params![new.name],
                |row| row.get(0)
            )?;
            if exists == 0 {
                self.insert_component(&new)?;
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

    // Palette Delegation
    pub fn insert_palette(&self, new: &NewPalette) -> Result<Palette> {
        palettes::insert_palette(&self.conn, new)
    }

    pub fn get_palette(&self, id: &str) -> Result<Palette> {
        palettes::get_palette(&self.conn, id)
    }

    pub fn list_palettes(&self) -> Result<Vec<Palette>> {
        palettes::list_palettes(&self.conn)
    }

    pub fn delete_palette(&self, id: &str) -> Result<()> {
        palettes::delete_palette(&self.conn, id)
    }

    pub fn seed_popular_palettes(&self) -> Result<usize> {
        palettes::seed_popular_palettes(&self.conn)
    }

    // Gradient Delegation
    pub fn insert_gradient(&self, new: &NewGradient) -> Result<Gradient> {
        gradients::insert_gradient(&self.conn, new)
    }

    pub fn get_gradient(&self, id: &str) -> Result<Gradient> {
        gradients::get_gradient(&self.conn, id)
    }

    pub fn list_gradients(&self) -> Result<Vec<Gradient>> {
        gradients::list_gradients(&self.conn)
    }

    pub fn delete_gradient(&self, id: &str) -> Result<()> {
        gradients::delete_gradient(&self.conn, id)
    }

    pub fn seed_popular_gradients(&self) -> Result<usize> {
        gradients::seed_popular_gradients(&self.conn)
    }
}
