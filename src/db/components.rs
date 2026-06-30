use crate::error::{OpenBlocksError, Result};
use crate::models::component::{Component, NewComponent, UpdateComponent};
use crate::models::{CategoryCount, FrameworkCount};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use uuid::Uuid;

/// Insert a new component into the database
pub fn insert_component(conn: &Connection, new: &NewComponent) -> Result<Component> {
    let id = Uuid::new_v4();
    let now = Utc::now().to_rfc3339();
    let deps_json = serde_json::to_string(&new.dependencies)?;
    let tags_json = serde_json::to_string(&new.tags)?;

    conn.execute(
        r#"INSERT INTO components
           (id, name, description, category, framework, code, dependencies, tags, version, created_at, updated_at)
           VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 1, ?9, ?10)"#,
        params![
            id.to_string(),
            new.name,
            new.description,
            new.category,
            new.framework,
            new.code,
            deps_json,
            tags_json,
            now,
            now,
        ],
    )?;

    get_component_by_id(conn, &id.to_string())
}

/// Get a single component by ID
pub fn get_component_by_id(conn: &Connection, id: &str) -> Result<Component> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, category, framework, code, dependencies, tags, preview_html, version, created_at, updated_at
         FROM components WHERE id = ?1"
    )?;

    let component = stmt.query_row(params![id], |row| {
        let created_str: String = row.get(10)?;
        let updated_str: String = row.get(11)?;

        let created_at = DateTime::parse_from_rfc3339(&created_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        let updated_at = DateTime::parse_from_rfc3339(&updated_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok(Component {
            id: row.get::<_, String>(0)?.parse().unwrap_or_default(),
            name: row.get(1)?,
            description: row.get(2)?,
            category: row.get::<_, String>(3)?.parse().unwrap_or_default(),
            framework: row.get::<_, String>(4)?.parse().unwrap_or_default(),
            code: row.get(5)?,
            dependencies: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
            tags: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
            preview_html: row.get(8)?,
            version: row.get(9)?,
            created_at,
            updated_at,
        })
    }).map_err(|_| OpenBlocksError::ComponentNotFound(id.to_string()))?;

    Ok(component)
}

/// Update an existing component (partial update)
pub fn update_component(conn: &Connection, update: &UpdateComponent) -> Result<Component> {
    // First verify component exists
    let existing = get_component_by_id(conn, &update.id)?;

    // Save current version to history
    let version_id = Uuid::new_v4();
    let now = Utc::now().to_rfc3339();

    conn.execute(
        r#"INSERT INTO component_versions (id, component_id, version, code, created_at)
           VALUES (?1, ?2, ?3, ?4, ?5)"#,
        params![
            version_id.to_string(),
            update.id,
            existing.version,
            existing.code,
            now,
        ],
    )?;

    // Build dynamic UPDATE query
    let new_version = existing.version + 1;
    let name = update.name.as_deref().unwrap_or(&existing.name);
    let description = update.description.as_deref().unwrap_or(&existing.description);
    let code = update.code.as_deref().unwrap_or(&existing.code);
    let existing_category = existing.category.to_string();
    let category = update.category.as_deref().unwrap_or(&existing_category);
    let existing_framework = existing.framework.to_string();
    let framework = update.framework.as_deref().unwrap_or(&existing_framework);

    let tags = match &update.tags {
        Some(t) => serde_json::to_string(t)?,
        None => serde_json::to_string(&existing.tags)?,
    };
    let deps = match &update.dependencies {
        Some(d) => serde_json::to_string(d)?,
        None => serde_json::to_string(&existing.dependencies)?,
    };

    conn.execute(
        r#"UPDATE components SET
           name = ?1, description = ?2, code = ?3, category = ?4, framework = ?5,
           tags = ?6, dependencies = ?7, version = ?8, updated_at = ?9
           WHERE id = ?10"#,
        params![name, description, code, category, framework, tags, deps, new_version, now, update.id],
    )?;

    get_component_by_id(conn, &update.id)
}

/// Delete a component by ID
pub fn delete_component(conn: &Connection, id: &str) -> Result<()> {
    // Verify it exists first
    let _ = get_component_by_id(conn, id)?;

    conn.execute("DELETE FROM components WHERE id = ?1", params![id])?;
    Ok(())
}

/// List components with optional filters
pub fn list_components(
    conn: &Connection,
    category: Option<&str>,
    framework: Option<&str>,
    limit: usize,
) -> Result<Vec<Component>> {
    let mut sql = String::from(
        "SELECT id, name, description, category, framework, code, dependencies, tags, preview_html, version, created_at, updated_at
         FROM components WHERE 1=1"
    );
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(cat) = category {
        sql.push_str(&format!(" AND category = ?{}", param_values.len() + 1));
        param_values.push(Box::new(cat.to_string()));
    }
    if let Some(fw) = framework {
        sql.push_str(&format!(" AND framework = ?{}", param_values.len() + 1));
        param_values.push(Box::new(fw.to_string()));
    }

    sql.push_str(" ORDER BY updated_at DESC");
    if limit != usize::MAX {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    let mut stmt = conn.prepare(&sql)?;
    let params: Vec<&dyn rusqlite::types::ToSql> = param_values.iter()
        .map(|p| p.as_ref())
        .collect();

    let rows = stmt.query_map(params.as_slice(), |row| {
        let created_str: String = row.get(10)?;
        let updated_str: String = row.get(11)?;

        let created_at = DateTime::parse_from_rfc3339(&created_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        let updated_at = DateTime::parse_from_rfc3339(&updated_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok(Component {
            id: row.get::<_, String>(0)?.parse().unwrap_or_default(),
            name: row.get(1)?,
            description: row.get(2)?,
            category: row.get::<_, String>(3)?.parse().unwrap_or_default(),
            framework: row.get::<_, String>(4)?.parse().unwrap_or_default(),
            code: row.get(5)?,
            dependencies: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
            tags: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
            preview_html: row.get(8)?,
            version: row.get(9)?,
            created_at,
            updated_at,
        })
    })?;

    let components: Vec<Component> = rows.filter_map(|r| r.ok()).collect();
    Ok(components)
}

/// List all components (for building search index)
pub fn list_all_components(conn: &Connection) -> Result<Vec<Component>> {
    list_components(conn, None, None, usize::MAX)
}

/// Get category counts
pub fn get_category_counts(conn: &Connection) -> Result<Vec<CategoryCount>> {
    let mut stmt = conn.prepare(
        "SELECT category, COUNT(*) FROM components GROUP BY category ORDER BY COUNT(*) DESC"
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(CategoryCount {
            category: row.get(0)?,
            count: row.get(1)?,
        })
    })?;
    let mut counts = Vec::new();
    for r in rows {
        counts.push(r?);
    }
    Ok(counts)
}

/// Get framework counts
pub fn get_framework_counts(conn: &Connection) -> Result<Vec<FrameworkCount>> {
    let mut stmt = conn.prepare(
        "SELECT framework, COUNT(*) FROM components GROUP BY framework ORDER BY COUNT(*) DESC"
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(FrameworkCount {
            framework: row.get(0)?,
            count: row.get(1)?,
        })
    })?;
    let mut counts = Vec::new();
    for r in rows {
        counts.push(r?);
    }
    Ok(counts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::open_connection;
    use crate::db::migrations::get_migrations;

    fn setup_test_db() -> Connection {
        let mut conn = open_connection(":memory:").unwrap();
        let migrations = get_migrations();
        migrations.to_latest(&mut conn).unwrap();
        conn
    }

    #[test]
    fn test_insert_and_get_component() {
        let conn = setup_test_db();
        let new_comp = NewComponent {
            name: "Test Navbar".into(),
            description: "A test navigation bar".into(),
            category: "navbar".into(),
            framework: "tailwind".into(),
            code: "<div>Navbar</div>".into(),
            tags: vec!["test".into(), "navbar".into()],
            dependencies: vec![],
        };

        let inserted = insert_component(&conn, &new_comp).unwrap();
        assert_eq!(inserted.name, "Test Navbar");
        assert_eq!(inserted.category.to_string(), "navbar");
        assert_eq!(inserted.framework.to_string(), "tailwind");

        let fetched = get_component_by_id(&conn, &inserted.id.to_string()).unwrap();
        assert_eq!(fetched.id, inserted.id);
        assert_eq!(fetched.name, "Test Navbar");
    }

    #[test]
    fn test_update_component() {
        let conn = setup_test_db();
        let new_comp = NewComponent {
            name: "Old Name".into(),
            description: "Old description".into(),
            category: "card".into(),
            framework: "css".into(),
            code: "<div>Old</div>".into(),
            tags: vec!["card".into()],
            dependencies: vec![],
        };

        let inserted = insert_component(&conn, &new_comp).unwrap();
        assert_eq!(inserted.version, 1);

        let update = UpdateComponent {
            id: inserted.id.to_string(),
            name: Some("New Name".into()),
            description: None,
            category: None,
            framework: None,
            code: Some("<div>New</div>".into()),
            tags: None,
            dependencies: None,
        };

        let updated = update_component(&conn, &update).unwrap();
        assert_eq!(updated.name, "New Name");
        assert_eq!(updated.description, "Old description");
        assert_eq!(updated.code, "<div>New</div>");
        assert_eq!(updated.version, 2);
    }
}

