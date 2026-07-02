use crate::error::{OpenBlocksError, Result};
use crate::models::template::{NewTemplate, Template};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use uuid::Uuid;

/// Insert a new template into the database
pub fn insert_template(conn: &Connection, new: &NewTemplate) -> Result<Template> {
    let id = Uuid::new_v4();
    let now = Utc::now().to_rfc3339();
    let layout_json = serde_json::to_string(&new.layout)?;
    let component_ids_json = serde_json::to_string(&new.component_ids)?;
    let variables_json = serde_json::to_string(&new.variables)?;

    conn.execute(
        r#"INSERT INTO templates
           (id, name, description, layout, component_ids, variables, created_at, updated_at)
           VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"#,
        params![
            id.to_string(),
            new.name,
            new.description,
            layout_json,
            component_ids_json,
            variables_json,
            now,
            now,
        ],
    )?;

    get_template_by_id(conn, &id.to_string())
}

/// Get a single template by ID
pub fn get_template_by_id(conn: &Connection, id: &str) -> Result<Template> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, layout, component_ids, variables, created_at, updated_at
         FROM templates WHERE id = ?1",
    )?;

    let template = stmt
        .query_row(params![id], |row| {
            let created_str: String = row.get(6)?;
            let updated_str: String = row.get(7)?;

            let created_at = DateTime::parse_from_rfc3339(&created_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            let updated_at = DateTime::parse_from_rfc3339(&updated_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            let component_ids_str: String = row.get(4)?;
            let component_ids_raw: Vec<String> =
                serde_json::from_str(&component_ids_str).unwrap_or_default();
            let component_ids: Vec<Uuid> = component_ids_raw
                .iter()
                .filter_map(|s| s.parse().ok())
                .collect();

            Ok(Template {
                id: row.get::<_, String>(0)?.parse().unwrap_or_default(),
                name: row.get(1)?,
                description: row.get(2)?,
                layout: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
                component_ids,
                variables: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
                created_at,
                updated_at,
            })
        })
        .map_err(|_| OpenBlocksError::TemplateNotFound(id.to_string()))?;

    Ok(template)
}

/// List all templates
pub fn list_templates(conn: &Connection) -> Result<Vec<Template>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, layout, component_ids, variables, created_at, updated_at
         FROM templates ORDER BY updated_at DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        let created_str: String = row.get(6)?;
        let updated_str: String = row.get(7)?;

        let created_at = DateTime::parse_from_rfc3339(&created_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        let updated_at = DateTime::parse_from_rfc3339(&updated_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        let component_ids_str: String = row.get(4)?;
        let component_ids_raw: Vec<String> =
            serde_json::from_str(&component_ids_str).unwrap_or_default();
        let component_ids: Vec<Uuid> = component_ids_raw
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect();

        Ok(Template {
            id: row.get::<_, String>(0)?.parse().unwrap_or_default(),
            name: row.get(1)?,
            description: row.get(2)?,
            layout: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
            component_ids,
            variables: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
            created_at,
            updated_at,
        })
    })?;

    let mut templates = Vec::new();
    for r in rows {
        templates.push(r?);
    }
    Ok(templates)
}

/// Delete a template
pub fn delete_template(conn: &Connection, id: &str) -> Result<()> {
    let _ = get_template_by_id(conn, id)?;
    conn.execute("DELETE FROM templates WHERE id = ?1", params![id])?;
    Ok(())
}
