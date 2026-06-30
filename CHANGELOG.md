# 📝 Changelog

All notable changes to the OpenBlocks project will be documented in this file.

---

## [0.0.1] - 2026-06-30

### Added
- **Initial release of OpenBlocks** — A Rust-native Model Context Protocol (MCP) server for web development.
- **Database Schema & Migrations**: Integrated SQLite connection layer with WAL mode and foreign key constraints enabled, utilizing `rusqlite_migration` to manage initial component and template tables.
- **Data Models**: Defined robust representations for components, category enums, framework enums, and layout templates with strict input validation rules.
- **Fuzzy Search Index**: Integrated `simsearch` to perform sub-millisecond local fuzzy matching against component names, descriptions, and tags.
- **15 Core MCP Tools**:
  - Components discovery: `search_components`, `get_component`, `list_categories`, `list_frameworks`, `get_stats`.
  - Components management: `add_component`, `update_component`, `delete_component`.
  - Templates management: `list_templates`, `get_template`, `add_template`, `delete_template`.
  - Scaffolding layout: `scaffold_page`.
  - Backup & restoration: `import_components`, `export_components`.
- **Template Rendering Pipeline**: Integrated `minijinja` engine to compile layout slots and inject user-customizable template variables into scaffolded HTML.
- **CLI Commands**:
  - `serve`: Starts stdio transport communications for MCP clients (logs strictly directed to `stderr` to prevent JSON-RPC channel pollution).
  - `seed`: Hydrates the database with default components.
  - `stats`: Generates quick terminal-based database counts.
- **Starter Components Seed**: Initial library seeding containing responsive dark navbar, gradient centered hero section, and minimalist footer.
