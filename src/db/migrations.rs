use rusqlite_migration::{Migrations, M};

pub fn get_migrations() -> Migrations<'static> {
    Migrations::new(vec![
        // Migration 1: Initial schema
        M::up(r#"
            CREATE TABLE IF NOT EXISTS components (
                id              TEXT PRIMARY KEY NOT NULL,
                name            TEXT NOT NULL,
                description     TEXT NOT NULL,
                category        TEXT NOT NULL,
                framework       TEXT NOT NULL,
                code            TEXT NOT NULL,
                dependencies    TEXT NOT NULL DEFAULT '[]',
                tags            TEXT NOT NULL DEFAULT '[]',
                preview_html    TEXT,
                version         INTEGER NOT NULL DEFAULT 1,
                created_at      TEXT NOT NULL,
                updated_at      TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS component_versions (
                id              TEXT PRIMARY KEY NOT NULL,
                component_id    TEXT NOT NULL,
                version         INTEGER NOT NULL,
                code            TEXT NOT NULL,
                description     TEXT,
                created_at      TEXT NOT NULL,
                FOREIGN KEY (component_id) REFERENCES components(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS templates (
                id              TEXT PRIMARY KEY NOT NULL,
                name            TEXT NOT NULL,
                description     TEXT NOT NULL,
                layout          TEXT NOT NULL DEFAULT '{}',
                component_ids   TEXT NOT NULL DEFAULT '[]',
                variables       TEXT NOT NULL DEFAULT '{}',
                created_at      TEXT NOT NULL,
                updated_at      TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_components_category ON components(category);
            CREATE INDEX IF NOT EXISTS idx_components_framework ON components(framework);
            CREATE INDEX IF NOT EXISTS idx_components_name ON components(name);
            CREATE INDEX IF NOT EXISTS idx_components_updated ON components(updated_at);
            CREATE INDEX IF NOT EXISTS idx_component_versions_cid ON component_versions(component_id);
            CREATE INDEX IF NOT EXISTS idx_templates_name ON templates(name);
        "#),
        // Migration 2: Add palettes table
        M::up(r#"
            CREATE TABLE IF NOT EXISTS palettes (
                id              TEXT PRIMARY KEY NOT NULL,
                name            TEXT NOT NULL,
                colors          TEXT NOT NULL DEFAULT '[]',
                tags            TEXT NOT NULL DEFAULT '[]',
                created_at      TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_palettes_name ON palettes(name);
        "#),
    ])
}
