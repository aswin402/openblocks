# 📋 OpenBlocks — Product Requirements Document (PRD)

**Version:** 1.0
**Date:** 2026-06-30
**Author:** OpenBlocks Team
**Status:** Draft

---

## 1. Executive Summary

**OpenBlocks** is a Rust-native MCP (Model Context Protocol) server that provides AI coding agents with a local, curated library of pre-built UI components, code blocks, and website templates. It exposes CRUD operations over the MCP protocol, enabling AI agents to search, retrieve, add, update, and delete components — transforming web development from "generate everything from scratch" to "compose from proven building blocks."

**Key value proposition:** AI agents produce better websites, faster, with consistent quality — because they assemble from curated components instead of inventing UI code from scratch.

---

## 2. Problem Statement

### The current state of AI web development is broken:

1. **No persistent component memory** — Every AI session starts from zero. Components generated yesterday are forgotten today. There's no reuse, no learning, no accumulation of quality.

2. **Inconsistent quality** — AI generates different HTML/CSS patterns for similar components across sessions. A navbar built today looks and behaves differently from one built tomorrow, even for the same project.

3. **Slow for complex UIs** — Generating a full landing page from scratch requires writing 500-1000+ lines of boilerplate CSS/HTML. Most of this code follows well-known patterns that don't need reinvention.

4. **No design system awareness** — AI agents don't know about design systems, spacing scales, color tokens, or typography hierarchies. They make ad-hoc decisions that look amateur.

5. **Framework fragmentation** — Projects use different CSS approaches (Tailwind, vanilla CSS, SCSS, Shadcn). AI agents handle each inconsistently and can't easily switch between them.

6. **No quality gate** — There's no curation step between AI-generated code and final usage. Bad patterns propagate silently.

### The cost:
- Developers spend **30-60% of their time fixing** AI-generated UI code
- Design consistency requires **manual review of every component**
- Switching CSS frameworks mid-project means **regenerating everything**
- No way to build a reusable library across projects

---

## 3. Target Users

### Primary: AI Coding Agents
- Claude (Desktop & API)
- Cursor IDE
- VS Code + Cline / Roo Code
- Windsurf
- GitHub Copilot (future MCP support)
- Any MCP-compatible AI agent

**How they use it:** AI agents call OpenBlocks MCP tools to search for components, retrieve code, and add new components back to the library during web development tasks.

### Secondary: Developers
- Frontend developers who curate component libraries
- Full-stack developers who want consistent UI across projects
- Teams who want a shared component library for their AI tools
- Open-source contributors who want to share component collections

**How they use it:** Developers seed the library with high-quality components, review AI-contributed components, export/share libraries, and configure which components are available.

---

## 4. Product Vision

> A world where every AI coding agent has instant access to a curated, production-quality component library — making AI-built websites indistinguishable from human-designed ones.

**3-month vision:** A working MCP server with 50+ curated components, CRUD operations, fuzzy search, and multi-framework support. Used daily by early adopters with Claude Desktop and Cursor.

**6-month vision:** 200+ components with template system, community-contributed component packs, and integration guides for all major AI coding tools.

**12-month vision:** The standard component library for AI web development. Marketplace for component packs. Semantic search. Visual preview system.

---

## 5. Goals & Success Metrics

| Goal | Metric | Target |
|---|---|---|
| Fast retrieval | Search latency | < 50ms for 10,000 components |
| Fast retrieval | Get component latency | < 10ms |
| Relevant search | Search result relevance | 90%+ top-3 hit rate |
| Growing library | Components added per month | 20+ (organic growth) |
| Multi-framework | Framework coverage | 4 frameworks (Tailwind, CSS, SCSS, Shadcn) |
| Reliability | Uptime (no crashes) | 99.9% during active sessions |
| Easy setup | Time to first use | < 2 minutes |
| Small footprint | Binary size | < 15MB |
| Low resource usage | Memory at idle | < 20MB RAM |
| Data safety | Zero data loss | 0 component losses per year |

---

## 6. Core Features

### 6.1 Component Registry

The heart of OpenBlocks. A structured database of UI components with rich metadata.

**Component attributes:**
| Attribute | Type | Required | Description |
|---|---|---|---|
| `id` | UUID | Auto | Unique identifier |
| `name` | String | ✅ | Human-readable name (e.g., "Glass Navbar") |
| `description` | String | ✅ | What this component is and when to use it |
| `category` | Enum | ✅ | UI category (navbar, hero, footer, card, etc.) |
| `framework` | Enum | ✅ | CSS framework (tailwind, css, scss, shadcn) |
| `code` | String | ✅ | The actual HTML/CSS/JS code |
| `tags` | String[] | ✅ | Searchable keywords |
| `dependencies` | String[] | Optional | External dependencies (e.g., ["tailwindcss@4"]) |
| `preview_html` | String | Optional | Standalone HTML for preview rendering |
| `version` | Integer | Auto | Version counter, increments on update |
| `created_at` | DateTime | Auto | Creation timestamp |
| `updated_at` | DateTime | Auto | Last modification timestamp |

**CRUD operations:**
- **Create** — AI or developer adds a new component with metadata
- **Read** — Search, get by ID, list by category/framework
- **Update** — Modify code, tags, description, or metadata
- **Delete** — Remove a component from the library

### 6.2 Smart Search

AI agents need to find the right component quickly and accurately.

**Search capabilities:**
- **Fuzzy text search** — "navbr" matches "navbar", "nav bar", "navigation bar"
- **Category filter** — "show me all hero sections"
- **Framework filter** — "tailwind cards only"
- **Tag filter** — "dark-mode, responsive, animated"
- **Combined filters** — "tailwind hero section with dark mode"
- **Relevance scoring** — Best matches first

### 6.3 Template System

Full-page templates composed of multiple components with layout definitions.

**Template attributes:**
| Attribute | Type | Description |
|---|---|---|
| `id` | UUID | Unique identifier |
| `name` | String | Template name (e.g., "SaaS Landing Page") |
| `description` | String | What this template is for |
| `layout` | JSON | Layout definition with component slots |
| `component_ids` | UUID[] | Components used in this template |
| `variables` | JSON | Customizable variables (colors, text, images) |

**Scaffolding:** AI agents can generate a complete page by selecting a template and providing customizations (brand colors, copy, images).

### 6.4 Multi-Framework Support

| Framework | Description | File Types |
|---|---|---|
| **Tailwind CSS** | Utility-first CSS framework | HTML with Tailwind classes |
| **Vanilla CSS** | Pure CSS, no framework | HTML + CSS |
| **SCSS** | CSS preprocessor with variables/mixins | HTML + SCSS |
| **Shadcn UI** | React components with Tailwind | TSX/JSX + Tailwind |

Components can exist in multiple framework variants. AI agents specify which framework they want when searching.

### 6.5 Import/Export

- **Import from JSON** — Bulk import component collections
- **Import from directory** — Scan a folder of HTML/CSS files
- **Export to JSON** — Export components for sharing or backup
- **Community packs** — Pre-built collections (e.g., "SaaS Starter Pack", "E-commerce Components")

### 6.6 Version Tracking

- Every update increments the version counter
- Previous versions stored in `component_versions` table
- AI can request a specific version or always get latest

---

## 7. MCP Tools Specification

### 7.1 Discovery Tools

#### `search_components`
Search the component library with fuzzy matching and filters.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `query` | string | ✅ | Search text (fuzzy matched) |
| `category` | string | ❌ | Filter by category |
| `framework` | string | ❌ | Filter by framework |
| `tags` | string[] | ❌ | Filter by tags |
| `limit` | integer | ❌ | Max results (default: 10) |

**Returns:** Array of `{ id, name, description, category, framework, tags, version }`

#### `get_component`
Get full component details including code.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | string (UUID) | ✅ | Component ID |

**Returns:** Complete component object with code

#### `list_categories`
List all available component categories with counts.

**Returns:** Array of `{ category, count }`

#### `list_frameworks`
List all supported frameworks with counts.

**Returns:** Array of `{ framework, count }`

#### `get_stats`
Get library-wide statistics.

**Returns:** `{ total_components, total_templates, categories, frameworks, last_updated }`

### 7.2 Mutation Tools

#### `add_component`
Add a new component to the library.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `name` | string | ✅ | Component name |
| `description` | string | ✅ | What it does and when to use it |
| `category` | string | ✅ | Category (navbar, hero, footer, etc.) |
| `framework` | string | ✅ | Framework (tailwind, css, scss, shadcn) |
| `code` | string | ✅ | The HTML/CSS/JS code |
| `tags` | string[] | ✅ | Searchable tags |
| `dependencies` | string[] | ❌ | External dependencies |

**Returns:** `{ id, name, message: "Component added successfully" }`

#### `update_component`
Update an existing component (partial updates supported).

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | string (UUID) | ✅ | Component to update |
| `name` | string | ❌ | New name |
| `description` | string | ❌ | New description |
| `code` | string | ❌ | New code |
| `tags` | string[] | ❌ | New tags |
| `category` | string | ❌ | New category |
| `framework` | string | ❌ | New framework |

**Returns:** `{ id, name, version, message: "Component updated" }`

#### `delete_component`
Remove a component from the library.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | string (UUID) | ✅ | Component to delete |

**Returns:** `{ id, message: "Component deleted" }`

### 7.3 Template Tools

#### `list_templates`
List all available page templates.

**Returns:** Array of `{ id, name, description, component_count }`

#### `get_template`
Get full template details.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | string (UUID) | ✅ | Template ID |

**Returns:** Complete template with layout definition and component references

#### `scaffold_page`
Generate a complete page from a template with customizations.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `template_id` | string (UUID) | ✅ | Template to use |
| `variables` | object | ❌ | Customization values (colors, text, etc.) |

**Returns:** `{ html, css, metadata }`

#### `add_template`
Create a new page template.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `name` | string | ✅ | Template name |
| `description` | string | ✅ | What this template is for |
| `component_ids` | string[] | ✅ | Component IDs in order |
| `layout` | object | ✅ | Layout definition |
| `variables` | object | ❌ | Customizable variable definitions |

**Returns:** `{ id, name, message }`

### 7.4 I/O Tools

#### `import_components`
Import components from a JSON file.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `file_path` | string | ✅ | Path to JSON file |

**Returns:** `{ imported: count, skipped: count, errors: [] }`

#### `export_components`
Export components to JSON.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `category` | string | ❌ | Filter by category |
| `framework` | string | ❌ | Filter by framework |
| `output_path` | string | ✅ | Where to save the JSON |

**Returns:** `{ exported: count, file_path }`

---

## 8. Non-Functional Requirements

### 8.1 Performance
| Metric | Requirement |
|---|---|
| Search latency | < 50ms for 10,000 components |
| Get component | < 10ms |
| Add component | < 100ms |
| Delete component | < 50ms |
| Server startup | < 500ms |
| Memory at idle | < 20MB |

### 8.2 Storage
- SQLite database file, typically < 100MB for 10,000 components
- Database auto-created on first run
- Located at `~/.local/share/openblocks/openblocks.db` (Linux) or platform equivalent
- Configurable via `--db-path` flag

### 8.3 Reliability
- Graceful error handling — tool errors return MCP error responses, never crash the server
- Database transactions for all mutations
- WAL mode for concurrent read/write safety
- Automatic database backup before migrations

### 8.4 Security
- Input validation on all tool parameters
- SQL parameterized queries (no string concatenation — prevents SQL injection)
- File path validation for import/export (no path traversal)
- No network access (local-only by default)

### 8.5 Compatibility
- MCP specification compliant (latest stable version)
- Stdio transport (Claude Desktop, Cursor, VS Code)
- Tested with: Claude Desktop, Cursor, VS Code + Cline
- Platforms: Linux x86_64, macOS aarch64, macOS x86_64, Windows x86_64

---

## 9. User Stories

### AI Agent Stories

| # | Story | Priority |
|---|---|---|
| US-1 | As an AI agent, I want to **search for components by description** so I can find relevant UI blocks without knowing exact names | P0 |
| US-2 | As an AI agent, I want to **filter search by framework** so I get components matching the project's CSS approach | P0 |
| US-3 | As an AI agent, I want to **get the full code of a component** so I can include it in the website I'm building | P0 |
| US-4 | As an AI agent, I want to **add a new component** I just generated so it can be reused in future sessions | P0 |
| US-5 | As an AI agent, I want to **scaffold a full page from a template** so I can quickly create complete landing pages | P1 |
| US-6 | As an AI agent, I want to **see what categories exist** so I can understand the library's coverage | P1 |
| US-7 | As an AI agent, I want to **update a component's code** when I improve it, preserving the component's identity | P1 |
| US-8 | As an AI agent, I want to **search by tags** so I can find components with specific features (dark-mode, animated, responsive) | P1 |

### Developer Stories

| # | Story | Priority |
|---|---|---|
| US-9 | As a developer, I want to **import my existing component library** from JSON so I don't start from zero | P0 |
| US-10 | As a developer, I want to **export components** so I can share them with teammates or back them up | P1 |
| US-11 | As a developer, I want to **delete low-quality components** that AI added so the library stays curated | P1 |
| US-12 | As a developer, I want **zero-config setup** — just download the binary and run it | P0 |
| US-13 | As a developer, I want to **see library statistics** so I know how many components I have per category | P2 |
| US-14 | As a developer, I want to **seed the library with starter components** on first run so it's immediately useful | P1 |

---

## 10. Competitive Analysis

| Feature | OpenBlocks | shadcn MCP | 21st.dev Magic | Flowbite MCP | Tailkit MCP |
|---|---|---|---|---|---|
| **Language** | Rust (single binary) | TypeScript | TypeScript | TypeScript | TypeScript |
| **CRUD** | Full (Create, Read, Update, Delete) | Read-only | Read-only | Read-only | Read-only |
| **Frameworks** | Tailwind, CSS, SCSS, Shadcn | Shadcn only | Custom | Tailwind only | Tailwind only |
| **Storage** | Local SQLite | Remote API | Remote API | Remote API | Remote API |
| **Offline** | ✅ Yes | ❌ No | ❌ No | ❌ No | ❌ No |
| **Page scaffolding** | ✅ Templates | ❌ No | ⚠️ Limited | ❌ No | ❌ No |
| **Custom components** | ✅ Add your own | ❌ No | ❌ No | ❌ No | ❌ No |
| **Search** | Fuzzy + filters | Basic | Natural language | Basic | Basic |
| **Setup** | Download & run | npm install | npm install | npm install | npm install |
| **Node.js required** | ❌ No | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
| **Open source** | ✅ Yes | ✅ Yes | ❌ No | Partial | ❌ No |
| **Price** | Free | Free | Freemium | Freemium | Paid |

### Competitive advantages:
1. **Only Rust-native solution** — faster, smaller, no dependencies
2. **Only solution with full CRUD** — AI agents can grow the library
3. **Only solution supporting 4 CSS frameworks** in one server
4. **Only offline-first solution** — no internet required
5. **Only solution with page scaffolding** from templates

---

## 11. Risks & Mitigations

| Risk | Impact | Likelihood | Mitigation |
|---|---|---|---|
| MCP protocol changes | High | Medium | Pin to stable spec version, abstract transport layer |
| Low adoption | High | Medium | Seed with high-quality components, write excellent docs |
| AI adds low-quality components | Medium | High | Add quality score field, let developers flag/delete bad components |
| SQLite performance ceiling | Medium | Low | Profile early, add indexes, consider tantivy for search at scale |
| Component code injection | High | Low | Sanitize inputs, validate HTML structure, never execute stored code |
| rmcp crate breaking changes | Medium | Medium | Pin version, contribute upstream, abstract SDK interface |
| Competing MCP server emerges | Medium | Medium | Move fast, build community, focus on CRUD + multi-framework differentiators |
| Binary size too large | Low | Low | Use `strip`, `opt-level = "s"`, consider feature flags |

---

## 12. Release Phases

### Phase 1: MVP (Weeks 1-3)
**Goal:** Working MCP server with component CRUD and search

**Deliverables:**
- [ ] Rust project with all dependencies
- [ ] SQLite database with component schema
- [ ] Component CRUD (add, get, update, delete, list)
- [ ] Fuzzy search with category/framework filters
- [ ] Stdio transport (works with Claude Desktop)
- [ ] 30+ seed components (navbars, heroes, footers, cards)
- [ ] README with setup instructions
- [ ] Test with MCP Inspector

**Definition of done:** An AI agent (Claude Desktop) can search for "dark navbar tailwind", get code, and add a new component — all working end-to-end.

### Phase 2: Templates & Polish (Weeks 4-5)
**Goal:** Page scaffolding and production quality

**Deliverables:**
- [ ] Template system (list, get, add, scaffold)
- [ ] 5 page templates (landing, dashboard, auth, blog, portfolio)
- [ ] Import/export (JSON)
- [ ] Component versioning
- [ ] Library statistics (get_stats)
- [ ] Error handling polish
- [ ] Performance benchmarks
- [ ] Cross-platform binaries (Linux, macOS, Windows)

**Definition of done:** An AI agent can scaffold a complete landing page from a template, customize it, and the result is production-ready HTML.

### Phase 3: Advanced Features (Weeks 6-8)
**Goal:** Power user features and community

**Deliverables:**
- [ ] Bulk import from directories
- [ ] Component preview (rendered HTML)
- [ ] Tag management
- [ ] SSE/HTTP transport option
- [ ] Community component packs
- [ ] GitHub Actions CI/CD
- [ ] Publish to crates.io
- [ ] Demo video and blog post

**Definition of done:** OpenBlocks is published, installable, documented, and has an active GitHub repository with community contributions.

---

## Appendix A: Category Taxonomy

| Category | Examples |
|---|---|
| `navbar` | Top navigation, hamburger menu, mega menu |
| `hero` | Hero section, hero with image, hero with video |
| `footer` | Simple footer, mega footer, footer with newsletter |
| `sidebar` | Dashboard sidebar, collapsible sidebar |
| `card` | Product card, profile card, pricing card, blog card |
| `form` | Login form, contact form, multi-step form |
| `modal` | Confirmation modal, form modal, image modal |
| `table` | Data table, sortable table, responsive table |
| `pricing` | Pricing tiers, comparison table, pricing toggle |
| `testimonial` | Testimonial slider, testimonial grid, single quote |
| `cta` | Call-to-action banner, CTA with image, inline CTA |
| `feature` | Feature grid, feature list, feature comparison |
| `faq` | Accordion FAQ, two-column FAQ |
| `contact` | Contact form, contact info, map section |
| `auth` | Login page, signup page, forgot password |
| `dashboard` | Dashboard layout, stats cards, chart section |
| `settings` | Settings page, profile settings, toggle settings |
| `profile` | User profile, team member card |
| `landing` | Full landing page section, above-the-fold |
| `blog` | Blog post layout, blog grid, blog sidebar |
| `ecommerce` | Product grid, cart, checkout, product detail |
| `error` | 404 page, 500 page, maintenance page |
| `loading` | Skeleton screen, spinner, progress bar |
| `notification` | Toast, alert banner, notification dropdown |

## Appendix B: Framework Identifiers

| Identifier | Framework | Code Format |
|---|---|---|
| `tailwind` | Tailwind CSS (v3/v4) | HTML with Tailwind utility classes |
| `css` | Vanilla CSS | HTML + separate CSS |
| `scss` | SCSS/Sass | HTML + SCSS |
| `shadcn` | Shadcn UI (React) | TSX/JSX + Tailwind |
