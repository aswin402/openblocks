# ✅ OpenBlocks — Project TODO & Roadmap

**Last Updated:** 2026-06-30
**Status:** Phases 0-8 Complete → Phase 9 Ready

---
## Quick Status

| Phase | Status | Priority | Est. Days | Depends On |
|---|---|---|---|---|
| Phase 0: Research & Planning | ✅ DONE | P0 | 1 | — |
| Phase 1: Project Foundation | ✅ DONE | P0 | 1-2 | Phase 0 |
| Phase 2: Database Layer | ✅ DONE | P0 | 2-3 | Phase 1 |
| Phase 3: Data Models | ✅ DONE | P0 | 1 | Phase 1 |
| Phase 4: Search Engine | ✅ DONE | P0 | 1-2 | Phase 3 |
| Phase 5: MCP Server Core | ✅ DONE | P0 | 2-3 | Phase 2, 3, 4 |
| Phase 6: Template System | ✅ DONE | P1 | 2 | Phase 5 |
| Phase 7: Import/Export & Seed Data | ✅ DONE | P1 | 2-3 | Phase 5 |
| Phase 8: Testing & Polish | ✅ DONE | P0 | 2-3 | Phase 5 |
| Phase 9: Distribution & Launch | 🔲 TODO | P1 | 2-3 | Phase 8 |
| Backlog: Future Ideas | 🔲 LATER | P2 | Ongoing | Phase 9 |

**Total estimated:** 16-23 days of focused work

---

## Phase 0: Research & Planning ✅ DONE

- [x] Research MCP protocol and available SDKs
- [x] Identify `rmcp` as the official Rust MCP SDK
- [x] Research competitor MCP servers (shadcn MCP, 21st.dev, Flowbite, Tailkit)
- [x] Select technology stack (Rust + rmcp + rusqlite + simsearch + minijinja)
- [x] Research Rust crates for each layer (storage, search, templating, serialization)
- [x] Create research report
- [x] Create Core Idea document (`coreidea.md`)
- [x] Create Product Requirements Document (`prd.md`)
- [x] Create Technical Specification (`spec.md`)
- [x] Create Implementation Guide (`implementations.md`)
- [x] Create TODO/Roadmap (`todo.md`) — this file

**✅ Definition of done:** All planning documents exist and are comprehensive enough to start coding.

---

## Phase 1: Project Foundation 🔲

**Priority:** P0 — Must complete first
**Estimated effort:** 1-2 days
**Depends on:** Phase 0

- [ ] Initialize Cargo project
  ```bash
  cd openblocks && cargo init --name openblocks
  ```
- [ ] Set up `Cargo.toml` with all dependencies:
  - [ ] `rmcp` 0.16 (server, transport-io, macros)
  - [ ] `tokio` 1.x (full)
  - [ ] `serde` + `serde_json` 1.x
  - [ ] `schemars` 0.8
  - [ ] `rusqlite` 0.33 (bundled)
  - [ ] `rusqlite_migration` 1.x
  - [ ] `simsearch` 0.2
  - [ ] `minijinja` 2.x
  - [ ] `tracing` + `tracing-subscriber`
  - [ ] `uuid` 1.x (v4, serde)
  - [ ] `chrono` 0.4 (serde)
  - [ ] `thiserror` 2.x
  - [ ] `anyhow` 1.x
  - [ ] `dirs` 6.x
  - [ ] `clap` 4.x (derive)
- [ ] Set up release profile (LTO, strip, opt-level z, panic abort)
- [ ] Create directory structure:
  - [ ] `src/db/` (mod.rs, connection.rs, migrations.rs, components.rs, templates.rs)
  - [ ] `src/models/` (mod.rs, component.rs, template.rs, enums.rs)
  - [ ] `src/search/` (mod.rs, engine.rs)
  - [ ] `src/server.rs`
  - [ ] `src/config.rs`
  - [ ] `src/error.rs`
  - [ ] `data/`
  - [ ] `tests/`
- [ ] Implement `src/main.rs`:
  - [ ] CLI arg parsing with clap (serve, seed, stats subcommands)
  - [ ] `--db-path`, `--log-level` flags
  - [ ] Env var support (`OPENBLOCKS_DB_PATH`, `OPENBLOCKS_LOG_LEVEL`)
- [ ] Configure tracing/logging:
  - [ ] **CRITICAL:** All logging to stderr, NEVER stdout
  - [ ] EnvFilter for log level control
  - [ ] Structured output format
- [ ] Implement `src/error.rs`:
  - [ ] `OpenBlocksError` enum with thiserror
  - [ ] Variants: ComponentNotFound, TemplateNotFound, InvalidCategory, InvalidFramework, Validation, Database, Json, Io, Import, Render
- [ ] Implement `src/config.rs`:
  - [ ] Config struct with database path, search defaults, transport mode
  - [ ] Load from `~/.config/openblocks/config.toml` (optional)
  - [ ] CLI args override config file
- [ ] Add `.gitignore` (Rust template + *.db + target/)
- [ ] Add `LICENSE` (MIT)
- [ ] Verify: `cargo build` compiles with empty module stubs

**✅ Definition of done:** `cargo build` succeeds. Running `./target/debug/openblocks --help` shows CLI usage. Logging goes to stderr.

---

## Phase 2: Database Layer 🔲

**Priority:** P0
**Estimated effort:** 2-3 days
**Depends on:** Phase 1

### Connection & Migrations
- [ ] Implement `src/db/connection.rs`:
  - [ ] `open_connection(path)` → `rusqlite::Connection`
  - [ ] Enable WAL mode (`PRAGMA journal_mode = WAL`)
  - [ ] Enable foreign keys (`PRAGMA foreign_keys = ON`)
  - [ ] Auto-create parent directories
  - [ ] Support `:memory:` for testing
- [ ] Implement `src/db/migrations.rs`:
  - [ ] Migration 1: Create `components` table
  - [ ] Migration 1: Create `component_versions` table
  - [ ] Migration 1: Create `templates` table
  - [ ] Migration 1: Create all indexes (category, framework, name, updated_at)
  - [ ] Test: migration runs on fresh DB without error
  - [ ] Test: migration is idempotent (running twice doesn't fail)

### Component CRUD
- [ ] Implement `src/db/components.rs`:
  - [ ] `insert_component(conn, NewComponent) → Result<Component>`
    - [ ] Generate UUID v4
    - [ ] Set created_at/updated_at to now
    - [ ] Serialize tags/dependencies to JSON
    - [ ] INSERT into components table
  - [ ] `get_component_by_id(conn, id) → Result<Component>`
    - [ ] SELECT by primary key
    - [ ] Deserialize tags/dependencies from JSON
    - [ ] Return ComponentNotFound error if missing
  - [ ] `list_components(conn, category?, framework?, limit) → Result<Vec<Component>>`
    - [ ] Dynamic WHERE clause based on filters
    - [ ] ORDER BY updated_at DESC
    - [ ] LIMIT clause
  - [ ] `list_all_components(conn) → Result<Vec<Component>>`
    - [ ] No filters, for building search index
  - [ ] `update_component(conn, UpdateComponent) → Result<Component>`
    - [ ] Verify component exists
    - [ ] Save current version to component_versions
    - [ ] Apply partial updates (only non-None fields)
    - [ ] Increment version counter
    - [ ] Update updated_at timestamp
  - [ ] `delete_component(conn, id) → Result<()>`
    - [ ] Verify exists, then DELETE
    - [ ] CASCADE deletes version history
  - [ ] `get_category_counts(conn) → Result<Vec<(String, i64)>>`
    - [ ] `SELECT category, COUNT(*) GROUP BY category`
  - [ ] `get_framework_counts(conn) → Result<Vec<(String, i64)>>`
    - [ ] `SELECT framework, COUNT(*) GROUP BY framework`

### Template CRUD
- [ ] Implement `src/db/templates.rs`:
  - [ ] `insert_template(conn, NewTemplate) → Result<Template>`
  - [ ] `get_template_by_id(conn, id) → Result<Template>`
  - [ ] `list_templates(conn) → Result<Vec<Template>>`
  - [ ] `delete_template(conn, id) → Result<()>`

### Database Wrapper
- [ ] Implement `src/db/mod.rs`:
  - [ ] `Database` struct wrapping `rusqlite::Connection`
  - [ ] `Database::new(path) → Result<Database>`
  - [ ] `Database::run_migrations() → Result<()>`
  - [ ] Delegate methods to components.rs and templates.rs
  - [ ] `get_stats() → Result<Stats>` (total components, templates, category/framework counts)

### Tests
- [ ] Write unit tests:
  - [ ] Test insert + get roundtrip
  - [ ] Test update increments version
  - [ ] Test update saves version history
  - [ ] Test partial update (only name changed, code unchanged)
  - [ ] Test delete removes component
  - [ ] Test delete of non-existent ID returns error
  - [ ] Test list with category filter
  - [ ] Test list with framework filter
  - [ ] Test list with limit
  - [ ] Test category/framework counts
  - [ ] Test template CRUD

**✅ Definition of done:** All CRUD tests pass with in-memory SQLite. `cargo test` is green.

---

## Phase 3: Data Models 🔲

**Priority:** P0
**Estimated effort:** 1 day
**Depends on:** Phase 1

- [ ] Implement `src/models/enums.rs`:
  - [ ] `Category` enum (26 variants: navbar, hero, footer, sidebar, card, form, modal, table, pricing, testimonial, cta, feature, faq, contact, auth, dashboard, settings, profile, landing, blog, ecommerce, error, loading, notification, section, other)
  - [ ] `impl Display for Category`
  - [ ] `impl FromStr for Category`
  - [ ] Serde rename_all = "lowercase"
  - [ ] `Framework` enum (tailwind, css, scss, shadcn)
  - [ ] `impl Display for Framework`
  - [ ] `impl FromStr for Framework`
- [ ] Implement `src/models/component.rs`:
  - [ ] `Component` struct (full model with all fields)
  - [ ] `NewComponent` struct (input for creation) with `JsonSchema` derive
  - [ ] `UpdateComponent` struct (partial update) with `JsonSchema` derive
  - [ ] `impl NewComponent::validate()` — validation logic
  - [ ] `SearchResult` struct (metadata without code)
- [ ] Implement `src/models/template.rs`:
  - [ ] `Template` struct
  - [ ] `NewTemplate` struct with `JsonSchema`
  - [ ] `ScaffoldRequest` struct with `JsonSchema`
- [ ] Implement `src/models/mod.rs`:
  - [ ] Re-export all models
  - [ ] `Stats` struct for get_stats
- [ ] Tests:
  - [ ] Test Category FromStr / Display roundtrip
  - [ ] Test Framework FromStr / Display roundtrip
  - [ ] Test NewComponent validation (valid input passes)
  - [ ] Test NewComponent validation (empty name fails)
  - [ ] Test NewComponent validation (empty code fails)
  - [ ] Test NewComponent validation (no tags fails)
  - [ ] Test NewComponent validation (invalid category fails)
  - [ ] Test NewComponent validation (invalid framework fails)
  - [ ] Test serde serialization/deserialization roundtrip

**✅ Definition of done:** All model tests pass. `JsonSchema` derives compile. Validation catches bad input.

---

## Phase 4: Search Engine 🔲

**Priority:** P0
**Estimated effort:** 1-2 days
**Depends on:** Phase 3

- [ ] Implement `src/search/engine.rs`:
  - [ ] `SearchEngine` struct wrapping `SimSearch<Uuid>`
  - [ ] `SearchEngine::new()` — create empty index
  - [ ] `index_component(component)` — add component to index
    - [ ] Combine name + description + tags into searchable string
  - [ ] `search(query) → Vec<Uuid>` — fuzzy search, return matching IDs
  - [ ] `rebuild(components)` — rebuild entire index from component list
- [ ] Implement `src/search/mod.rs`:
  - [ ] Re-export SearchEngine
- [ ] Tests:
  - [ ] Test: index 1 component, search finds it by name
  - [ ] Test: index 1 component, search finds it by tag
  - [ ] Test: search with typo finds component (fuzzy matching)
  - [ ] Test: empty query returns empty results
  - [ ] Test: search non-existent term returns empty results
  - [ ] Test: rebuild index works correctly
  - [ ] Test: index 100 components, search returns relevant results
- [ ] Performance benchmark:
  - [ ] Index 1000 components, search < 10ms
  - [ ] Index 10,000 components, search < 50ms

**✅ Definition of done:** Search tests pass. Fuzzy matching works (e.g., "navbr" finds "navbar"). Performance within targets.

---

## Phase 5: MCP Server Core 🔲

**Priority:** P0
**Estimated effort:** 2-3 days
**Depends on:** Phase 2, Phase 3, Phase 4

### Server Setup
- [ ] Implement `src/server.rs`:
  - [ ] `OpenBlocksServer` struct with `Mutex<Database>` + `Mutex<SearchEngine>`
  - [ ] `OpenBlocksServer::new(db, search) → Self`
  - [ ] `OpenBlocksServer::serve()` — stdio transport
  - [ ] `#[tool_router]` attribute on impl block

### MCP Tools Implementation
- [ ] **Discovery tools:**
  - [ ] `search_components(query, category?, framework?, limit?)` — fuzzy search with filters
  - [ ] `get_component(id)` — full component with code
  - [ ] `list_categories()` — categories with counts
  - [ ] `list_frameworks()` — frameworks with counts
  - [ ] `get_stats()` — library statistics
- [ ] **Mutation tools:**
  - [ ] `add_component(name, description, category, framework, code, tags, dependencies?)` — create new
  - [ ] `update_component(id, name?, description?, code?, tags?, category?, framework?)` — partial update
  - [ ] `delete_component(id)` — remove component

### Error Handling
- [ ] All tools return proper MCP responses (never panic)
- [ ] Invalid inputs return `CallToolResult::error` with helpful messages
- [ ] Database errors are caught and returned as MCP errors
- [ ] Invalid category/framework names return list of valid options

### Testing
- [ ] Build and run: `cargo build && ./target/debug/openblocks serve`
- [ ] Test with MCP Inspector: `npx @modelcontextprotocol/inspector ./target/debug/openblocks serve`
  - [ ] Verify all tools appear in the Inspector
  - [ ] Test `add_component` → verify success response
  - [ ] Test `search_components` → verify results
  - [ ] Test `get_component` → verify code in response
  - [ ] Test `update_component` → verify version increments
  - [ ] Test `delete_component` → verify deletion
  - [ ] Test `list_categories` → verify counts
  - [ ] Test `list_frameworks` → verify counts
  - [ ] Test `get_stats` → verify statistics
  - [ ] Test error cases (invalid ID, missing required fields)
- [ ] Test with Claude Desktop:
  - [ ] Add to `claude_desktop_config.json`
  - [ ] Verify tools appear in Claude's tool list
  - [ ] Ask Claude to search for a component
  - [ ] Ask Claude to add a new component
- [ ] Test with Cursor:
  - [ ] Add to `.cursor/mcp.json`
  - [ ] Verify tools appear
  - [ ] Run a basic workflow

**✅ Definition of done:** All 8 MCP tools work end-to-end. Tested with MCP Inspector AND at least one real AI client (Claude Desktop or Cursor).

---

## Phase 6: Template System 🔲

**Priority:** P1 — Should have
**Estimated effort:** 2 days
**Depends on:** Phase 5

- [ ] Define template layout schema (JSON format)
- [ ] Set up MiniJinja environment
- [ ] Add MCP tools:
  - [ ] `list_templates()` — list all templates with metadata
  - [ ] `get_template(id)` — get template details
  - [ ] `add_template(name, description, component_ids, layout, variables?)` — create template
  - [ ] `scaffold_page(template_id, variables?)` — render full page HTML
- [ ] Implement rendering pipeline:
  - [ ] Load template from DB
  - [ ] Load each referenced component by ID
  - [ ] Place components into layout slots
  - [ ] Apply variable substitutions
  - [ ] Return combined HTML
- [ ] Create starter templates:
  - [ ] SaaS Landing Page (navbar + hero + features + pricing + footer)
  - [ ] Dashboard Layout (sidebar + header + main content area)
  - [ ] Auth Page (centered form with branding)
  - [ ] Blog Layout (header + content + sidebar + footer)
  - [ ] Portfolio (navbar + hero + project grid + contact + footer)
- [ ] Tests:
  - [ ] Test scaffold_page produces valid HTML
  - [ ] Test variable substitution works
  - [ ] Test missing component ID returns error
  - [ ] Test empty template renders base HTML

**✅ Definition of done:** `scaffold_page` produces complete, valid HTML from templates. 5 starter templates exist.

---

## Phase 7: Import/Export & Seed Data 🔲

**Priority:** P1
**Estimated effort:** 2-3 days
**Depends on:** Phase 5

### Import/Export
- [ ] Add MCP tools:
  - [ ] `import_components(file_path)` — import from JSON file
  - [ ] `export_components(output_path, category?, framework?)` — export to JSON
- [ ] Implement JSON import:
  - [ ] Read file, parse JSON array of NewComponent
  - [ ] Validate each component
  - [ ] Insert valid components, skip invalid with error messages
  - [ ] Return count of imported/skipped/errors
- [ ] Implement JSON export:
  - [ ] Query components with optional filters
  - [ ] Serialize to pretty JSON
  - [ ] Write to file
- [ ] Test import/export roundtrip

### Seed Data Creation
- [ ] Create `data/seed_components.json` with 30+ starter components:
  - [ ] **Navbars (5):**
    - [ ] Simple dark navbar (Tailwind)
    - [ ] Glass morphism navbar (Tailwind)
    - [ ] Navbar with dropdown (CSS)
    - [ ] Responsive hamburger navbar (Tailwind)
    - [ ] Sidebar navigation (Tailwind)
  - [ ] **Hero Sections (5):**
    - [ ] Gradient hero with CTA (Tailwind)
    - [ ] Hero with background image (CSS)
    - [ ] Centered hero with animation (Tailwind)
    - [ ] Split hero (image + text) (Tailwind)
    - [ ] Video background hero (CSS)
  - [ ] **Footers (4):**
    - [ ] Simple footer with links (Tailwind)
    - [ ] Mega footer with columns (Tailwind)
    - [ ] Footer with newsletter signup (CSS)
    - [ ] Minimal footer (CSS)
  - [ ] **Cards (5):**
    - [ ] Product card with hover (Tailwind)
    - [ ] Blog post card (Tailwind)
    - [ ] Profile/team member card (CSS)
    - [ ] Pricing card (Tailwind)
    - [ ] Testimonial card (CSS)
  - [ ] **Pricing Sections (3):**
    - [ ] Three-tier pricing table (Tailwind)
    - [ ] Pricing with toggle (monthly/yearly) (Tailwind)
    - [ ] Comparison pricing table (CSS)
  - [ ] **CTA Sections (3):**
    - [ ] Banner CTA with gradient (Tailwind)
    - [ ] CTA with image (CSS)
    - [ ] Inline CTA (Tailwind)
  - [ ] **Auth Forms (3):**
    - [ ] Login form (Tailwind)
    - [ ] Signup form with social login (Tailwind)
    - [ ] Forgot password form (CSS)
  - [ ] **Feature Sections (3):**
    - [ ] Feature grid with icons (Tailwind)
    - [ ] Feature list with descriptions (CSS)
    - [ ] Bento grid features (Tailwind)
  - [ ] **FAQ (2):**
    - [ ] Accordion FAQ (Tailwind + JS)
    - [ ] Two-column FAQ (CSS)
- [ ] Implement `seed` CLI command:
  - [ ] Read seed_components.json
  - [ ] Insert all components (skip if DB already has data)
  - [ ] Build search index after seeding
- [ ] Auto-seed on first run (if DB is empty)

**✅ Definition of done:** 30+ high-quality seed components exist. `openblocks seed` populates the DB. Export → import roundtrip preserves all data.

---

## Phase 8: Testing & Polish 🔲

**Priority:** P0
**Estimated effort:** 2-3 days
**Depends on:** Phase 5 (can start in parallel with Phase 6-7)

### Code Quality
- [ ] Error handling audit:
  - [ ] Every tool returns MCP error responses, never panics
  - [ ] All error messages are human-readable and helpful
  - [ ] Invalid enum values return list of valid options
- [ ] Input validation audit:
  - [ ] SQL parameterized queries everywhere (no string concatenation)
  - [ ] File path validation for import/export (no path traversal: `..`)
  - [ ] Name/description length limits enforced
  - [ ] UUID format validation on all ID inputs
- [ ] Logging audit:
  - [ ] ZERO `println!` or `print!` calls in the entire codebase
  - [ ] All logging uses `tracing` macros
  - [ ] Sensitive data not logged (no full component code in logs)

### Test Suite
- [ ] Unit tests:
  - [ ] DB: insert, get, update, delete, list, filter (12+ tests)
  - [ ] Models: enum conversion, validation, serialization (10+ tests)
  - [ ] Search: indexing, fuzzy matching, rebuild (8+ tests)
- [ ] Integration tests:
  - [ ] Full MCP tool cycle (add → search → get → update → delete)
  - [ ] Import/export roundtrip
  - [ ] Template scaffold
  - [ ] Edge cases:
    - [ ] Empty database (search returns empty, list returns empty)
    - [ ] Duplicate component names (should work — names aren't unique keys)
    - [ ] Very long component code (10,000+ characters)
    - [ ] Special characters in names/descriptions
    - [ ] Non-existent IDs
    - [ ] Invalid category/framework strings
- [ ] `cargo test` — ALL tests pass
- [ ] `cargo clippy` — zero warnings
- [ ] `cargo fmt --check` — code is formatted

### Performance
- [ ] Benchmark search with 1,000 components (target: < 10ms)
- [ ] Benchmark search with 10,000 components (target: < 50ms)
- [ ] Benchmark get_component (target: < 10ms)
- [ ] Benchmark add_component (target: < 100ms)
- [ ] Benchmark server startup with 1,000 components (target: < 500ms)
- [ ] Measure idle memory usage (target: < 20MB)
- [ ] Measure release binary size (target: < 15MB)

### Documentation
- [ ] Write comprehensive `README.md`:
  - [ ] Project description and features
  - [ ] Quick start (download binary, run, configure AI client)
  - [ ] Installation from source
  - [ ] CLI usage and flags
  - [ ] MCP tool reference (all tools with examples)
  - [ ] Configuration guide
  - [ ] Seed data documentation
  - [ ] Contributing guidelines
- [ ] Update all docs (prd.md, spec.md, etc.) with actual implementation details

**✅ Definition of done:** `cargo test` passes. `cargo clippy` clean. Performance within targets. README is comprehensive.

---

## Phase 9: Distribution & Launch 🔲

**Priority:** P1
**Estimated effort:** 2-3 days
**Depends on:** Phase 8

### Build & Release
- [ ] Set up GitHub repository
- [ ] Add CI/CD with GitHub Actions:
  - [ ] Run `cargo test` on every PR
  - [ ] Run `cargo clippy` on every PR
  - [ ] Run `cargo fmt --check` on every PR
  - [ ] Build release binaries on tag push
- [ ] Cross-compile release binaries:
  - [ ] Linux x86_64 (`x86_64-unknown-linux-gnu`)
  - [ ] Linux aarch64 (`aarch64-unknown-linux-gnu`)
  - [ ] macOS Apple Silicon (`aarch64-apple-darwin`)
  - [ ] macOS Intel (`x86_64-apple-darwin`)
  - [ ] Windows (`x86_64-pc-windows-msvc`)
- [ ] Create GitHub Release with all binaries
- [ ] Write installation script (curl-based one-liner)

### Client Configuration Guides
- [ ] Claude Desktop setup guide (with screenshots)
- [ ] Cursor IDE setup guide
- [ ] VS Code + Cline setup guide
- [ ] VS Code + Roo Code setup guide
- [ ] Windsurf setup guide
- [ ] Generic MCP client guide

### Publishing
- [ ] Publish to crates.io (`cargo publish`)
- [ ] Add to awesome-mcp-servers list (PR to GitHub repo)
- [ ] Register on mcp.directory
- [ ] Register on mcpserverfinder.com

### Marketing
- [ ] Create demo GIF/video showing:
  - [ ] AI agent searching for components
  - [ ] AI agent building a page from components
  - [ ] AI agent adding a new component
- [ ] Write announcement blog post
- [ ] Post on Reddit (r/rust, r/webdev, r/programming)
- [ ] Post on Hacker News
- [ ] Post on X/Twitter
- [ ] Post on relevant Discord servers (Rust, MCP, AI coding)

**✅ Definition of done:** Binary downloadable from GitHub Releases. Listed on at least 2 MCP directories. Blog post published.

---

## Backlog: Future Ideas 🔲

**Priority:** P2 — Nice to have, post-launch
**No timeline — pick based on user feedback**

### Infrastructure
- [ ] SSE/HTTP transport for remote/shared access
- [ ] Web dashboard for browsing/managing components (HTML + JS, served by OpenBlocks)
- [ ] Component playground (live preview in browser via embedded HTTP server)
- [ ] Component screenshot generation (headless browser → PNG)
- [ ] Git integration (version components with git instead of/alongside version counter)

### Search & Discovery
- [ ] Semantic search with embeddings (local model or API)
- [ ] "Similar components" — find components similar to a given one
- [ ] Auto-tagging (analyze code to suggest tags)
- [ ] Auto-detect framework from code analysis
- [ ] Natural language queries ("I need a dark-themed pricing section with a toggle")

### Component Quality
- [ ] Quality score field (1-5 stars, set by developer)
- [ ] Component dependency graph (which components work well together)
- [ ] HTML validation (check components are valid HTML before storing)
- [ ] Responsive breakpoint annotations
- [ ] Accessibility score (basic a11y checks on stored components)

### Ecosystem
- [ ] Community component packs (downloadable JSON bundles):
  - [ ] SaaS Starter Pack
  - [ ] E-commerce Pack
  - [ ] Dashboard Pack
  - [ ] Portfolio Pack
  - [ ] Blog Pack
- [ ] Marketplace for sharing component packs
- [ ] Plugin system for custom frameworks (beyond Tailwind/CSS/SCSS/Shadcn)
- [ ] Tailwind v4 compatibility validation
- [ ] Shadcn registry protocol compatibility (serve as a shadcn-compatible registry)

### AI Integration
- [ ] AI-powered component suggestions ("based on your project, try these components")
- [ ] Component deduplication (detect near-duplicates before adding)
- [ ] Auto-generate descriptions from code
- [ ] Component composition suggestions ("this hero pairs well with this navbar")

### Developer Experience
- [ ] MCP Resources (expose component categories as MCP resources, not just tools)
- [ ] MCP Prompts (pre-built prompts like "build a landing page" that guide the AI)
- [ ] Config via MCP tool (let AI modify server config)
- [ ] Component edit history with diff view
- [ ] Backup/restore database

---

## Notes

### Priority Legend
| Priority | Meaning |
|---|---|
| **P0** | Must have for MVP — blocks launch |
| **P1** | Should have — significantly improves product |
| **P2** | Nice to have — can launch without it |

### How to Use This File
1. Work through phases **in order** (dependencies are real)
2. Check boxes as you complete tasks
3. Run the checkpoint at the end of each phase before moving on
4. Update the Quick Status table as phases complete
5. Move backlog items to active phases when ready

### Key Milestones
| Milestone | Definition | Target |
|---|---|---|
| **First compile** | `cargo build` succeeds | Phase 1 complete |
| **First CRUD** | Insert + Get + Update + Delete works | Phase 2 complete |
| **First search** | Fuzzy search returns relevant results | Phase 4 complete |
| **First MCP call** | AI agent successfully calls a tool | Phase 5 complete |
| **MVP** | All 8 core tools work with real AI client | Phase 5 complete |
| **Beta** | Templates + seed data + import/export | Phase 7 complete |
| **Launch** | Published, documented, downloadable | Phase 9 complete |
