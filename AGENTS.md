# OpenBlocks — Agent Guide

OpenBlocks is a local-first **MCP server** (Rust, rmcp) that exposes a library of UI components, color palettes, gradients, and website templates to AI coding agents via the Model Context Protocol (stdio transport).

## Quick Reference

```bash
# Build
cargo build                    # debug build
cargo build --release          # release with LTO/strip

# Run MCP server (default command)
cargo run -- serve

# Seed starter data
cargo run -- seed

# Show library stats
cargo run -- stats

# Custom DB path / log level
cargo run -- --db-path /tmp/my.db --log-level debug serve

# Test (only 2 tests exist, in db/components.rs)
cargo test

# Lint
cargo clippy
cargo fmt --check
```

## CLI

| Command | Description |
|---------|-------------|
| `serve` (default) | Start MCP server on stdio |
| `seed` | Insert seed data from `data/seed_components.json` + palettes + gradients |
| `stats` | Show library statistics |

Flags: `--db-path` (or `OPENBLOCKS_DB_PATH` env var), `--log-level` (or `OPENBLOCKS_LOG_LEVEL` env var, default `info`)

## Architecture

```
src/
├── main.rs          # CLI parser (clap), logger init, DB + search engine setup
├── server.rs        # OpenBlocksServer struct + all 15 MCP tools (#[tool_router])
├── config.rs        # Config struct (used by main but config file loading not wired)
├── error.rs         # OpenBlocksError enum (thiserror), custom Result<T>
├── models/
│   ├── mod.rs       # Re-exports all models, Stats/CategoryCount/FrameworkCount
│   ├── enums.rs     # Category (26 variants), Framework (4 variants), serde rename_all="lowercase"
│   ├── component.rs # Component, NewComponent, UpdateComponent, SearchResult
│   ├── template.rs  # Template, NewTemplate, ScaffoldRequest
│   ├── palette.rs   # Palette, NewPalette (exactly 4 hex colors)
│   └── gradient.rs  # Gradient, NewGradient (name + CSS + colors)
├── db/
│   ├── mod.rs       # Database struct wrapping rusqlite::Connection, delegates to submodules
│   ├── connection.rs# open_connection(), WAL mode + foreign_keys pragma
│   ├── migrations.rs# 3 migrations (components+versions, palettes, gradients)
│   ├── components.rs# Component CRUD, dynamic WHERE, category/framework counts
│   ├── templates.rs # Template CRUD
│   ├── palettes.rs  # Palette CRUD + seed_popular_palettes() (10 seeded)
│   └── gradients.rs # Gradient CRUD + seed_popular_gradients() (10 seeded)
├── search/
│   ├── mod.rs
│   └── engine.rs    # SimSearch<Uuid>, indexes name+description+tags
└── scripts/         # Python data-fetching scripts (not part of build)
    ├── fetch_tailblocks.py
    ├── fetch_uiverse.py
    └── fetch_webgradients.py
data/                # JSON seed files
    ├── seed_components.json       # 264 seed components (including tailblocks + uiverse imports)
    ├── css_text_animations.json   # 15 CSS text animation components
    ├── tailblocks_components.json
    ├── uiverse_components.json
```

## Control/Data Flow

1. **main.rs** parses CLI → opens DB → runs migrations → loads all components → builds SimSearch index
2. **server.rs** wraps DB + SearchEngine in `Arc<Mutex<...>>` and starts stdio MCP transport
3. MCP tools acquire locks on DB and/or SearchEngine per call
4. Components kept **in DB only** (not in-memory cache). Search index is rebuilt on modify/delete/import.
5. Templates reference components by UUID. `scaffold_page()` loads template → loads each component by ID → renders via MiniJinja.

## MCP Tools (15 total)

### Component Discovery
- `search_components(query, category?, framework?, limit?)` — fuzzy search, returns metadata (no code)
- `get_component(id)` — full component with code
- `list_categories()` / `list_frameworks()` / `get_stats()`

### Component CRUD
- `add_component(name, description, category, framework, code, tags, dependencies?)`
- `update_component(id, name?, description?, code?, tags?, category?, framework?, dependencies?)` — auto-increments version, saves old code to `component_versions`
- `delete_component(id)`

### Templates
- `list_templates()` / `get_template(id)` / `add_template(...)` / `delete_template(id)` / `scaffold_page(template_id, variables?)`

### Import/Export
- `import_components(file_path)` — JSON array of NewComponent, skips duplicates by name
- `export_components(output_path, category?, framework?)`

### Color Palettes & Gradients
- `list_palettes()` / `get_palette(id)` / `add_palette(name, colors[4], tags?)` / `delete_palette(id)`
  - Palette `validate()` requires **exactly 4** hex colors
- `list_gradients()` / `get_gradient(id)` / `add_gradient(name, css, colors, tags?)` / `delete_gradient(id)`

## Key Conventions & Gotchas

**CRITICAL — stdout is the MCP transport channel.** All logging, debug output, progress messages MUST go to stderr. `println!` or `print!` anywhere in the code will silently corrupt the MCP JSON-RPC stream and break the server. The project uses `tracing` with `with_writer(std::io::stderr)` to enforce this.

**Search index is rebuilt on mutation**, not incrementally updated. `update_component` and `delete_component` re-`list_all_components()`, then rebuild the entire SimSearch index. `add_component` uses `index_component()` for incremental insert. This is a performance gotcha for bulk mutations.

**Locks are held across async boundaries.** `self.inner.db.lock().unwrap()` blocks the tokio thread. For a single-connection stdio server this is fine, but if HTTP/SSE transport is added later, the `Mutex<Database>` pattern will become a contention bottleneck.

**Dates stored as RFC 3339 strings** (not SQLite datetime). `chrono::DateTime::parse_from_rfc3339().with_timezone(&Utc)` is used on read, with a silent fallback to `Utc::now()` on parse failure.

**Category/Framework enums use `#[serde(rename_all = "lowercase")]` + `FromStr` via serde JSON roundtrip.** Display: serde serializes → trims quotes. FromStr: wraps input in quotes → serde deserializes. Framework additionally normalises aliases (`tailwindcss`→`tailwind`, `vanillacss`→`css`, `sass`→`scss`, `shadcnui`→`shadcn`).

**Palettes have a hard requirement of exactly 4 hex colors.** Gradients require at least 1 color. Both use string UUIDs (not `uuid::Uuid`).

**No HTTP/SSE transport** — only stdio. The `ServerConfig.transport` and `bind_address` fields in `config.rs` are unused stubs.

**Tests are minimal** — only 2 unit tests exist (both in `db/components.rs`). No tests for search, models, palettes, gradients, or templates. The `tests/` directory doesn't exist.

**Seed behavior**: `seed_from_file()` skips components whose `name` already exists in the DB (checked via `SELECT COUNT(*)`). Palette and gradient seeding check if any rows exist and skip entirely if so. `seed` CLI command also seeds palettes + gradients + CSS text animations (from `data/css_text_animations.json`).

**Gradients**: 40 seeded gradients sourced from WebGradients (itmeo/webgradients). The existing `scripts/fetch_webgradients.py` can also fetch the full 100+ dataset directly into the SQLite DB.

**Database path defaults** to `${XDG_DATA_DIRS}/openblocks/openblocks.db` (or `./openblocks.db` if no data dir is resolvable). Useful to know when reading/testing without explicit `--db-path`.

**No `.pls` or makefile** — everything goes through `cargo`.

## Framework Aliases (FromStr normalization)

| Input | Normalized |
|-------|-----------|
| `tailwind`, `tailwindcss` | `tailwind` |
| `css`, `vanillacss` | `css` |
| `scss`, `sass` | `scss` |
| `shadcn`, `shadcnui` | `shadcn` |

## Testing

Only 2 tests exist in the project (both in `src/db/components.rs`):
- `test_insert_and_get_component`
- `test_update_component` (checks version goes from 1→2)

Both use `:memory:` SQLite. `setup_test_db()` helper: opens `:memory:`, runs all migrations, returns Connection.

When adding tests: use the same `setup_test_db()` pattern. Search tests need `SimSearch::new()`. Model tests test FromStr/Display and validation methods.
