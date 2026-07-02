# 📝 Changelog

All notable changes to the OpenBlocks project will be documented in this file.

---

## [0.0.3] - 2026-07-02

### Added
- **React Bits & Community Components**: Added `data/react_bits_components.json` containing 4 premium components from React Bits, React Components, and official Shadcn Blocks (Tilted Card, True Focus Text, Glass Time Card, and Shadcn UI Sidebar Navigation).
- **React Animated Components**: Added a new database seed file `data/animated_components.json` containing 6 premium interactive React blocks inspired by Motion Primitives, Aceternity UI, and BuildUI (Sparkles Background, Magnified Dock, Text Reveal, Glowing Grid, Resizable Panel, Magnetic Hover).
- **Shadcn UI React Components**: Added a new database seed file `data/shadcn_components.json` containing 6 premium, responsive React + Tailwind CSS blocks (Product Card, Login Form, Pricing Deck, Hero Banner, Bento Grid, Accordion FAQ).
- **GitHub Actions Workflows (CI/CD)**: Added `.github/workflows/ci.yml` (automating `cargo fmt`, `cargo clippy`, and `cargo test` checks) and `.github/workflows/release.yml` (cross-compiling and packaging the server release for Linux x86_64/aarch64, macOS Apple Silicon/Intel, and Windows on version tag push).
- **Automated Installer Script**: Added `scripts/install.sh` script to auto-detect platform, download the pre-compiled binary from GitHub releases, and run database initialization/seeding.
- **Project Logo**: Created an animated SVG logo banner (`logo.svg`) showcasing isometric cascading blocks styled with premium gradients and pulsing drop-shadow keyframe effects.

### Changed
- **Client Configuration Guide**: Updated client configuration instructions for Claude Desktop and Cursor IDE to use the pre-compiled binary by default.
- **Codebase Linting**: Resolved multiple `cargo clippy` suggestions (collapsible nested `if` statements) and reformatted code with `cargo fmt`.

---

## [0.0.2] - 2026-06-30

### Added
- **CSS Button Collection**: Added 92 real-world CSS button components sourced from CSS Scan (getcssscan.com/css-buttons-examples), featuring buttons from Dribbble, GitHub, Sketch, Stackoverflow, Stripe, Apple, Google, LinkedIn, Duolingo, Airbnb, Linear, MongoDB, Tailwind, Chakra UI, Bulma, Material, Loom, and many more brands.
- **Color Gradients Support**: Expanded seeded gradients from 10 to 40 with diverse color combinations inspired by WebGradients.com.
- **CSS Text Animation Collection**: Seeded 40 CSS text animation components (typewriter, glitch, neon, 3D, kinetic, and more) sourced from CodePen examples and creative patterns.
- **HeroUI React Components**: Added 25 production-grade React/JSX components inspired by HeroUI (Button, Card, Modal, Input, Accordion, Alert, Avatar, Badge, Tabs, Select, Tooltip, Spinner, ProgressBar, Skeleton, Switch, Checkbox, Breadcrumbs, Pagination, Chip, Dropdown, Table, Toast, Link, TextArea, Drawer).
- **"react" Framework Enum**: Extended the `Framework` enum to support React/JSX/TSX components alongside tailwind, css, scss, and shadcn.
- **Seed Data Expansion**: Database seeding now loads from 6 JSON data files plus built-in palettes and gradients (264 base + 44 tailblocks + 24 uiverse + 40 text animations + 25 HeroUI + 92 CSS buttons + 10 palettes + 40 gradients).

### Changed
- **Library Total**: Over 530 pre-built UI components available after seeding.
- **Seed CLI command**: Seeds all data sources automatically.
- **Data Extraction Script**: Added `scripts/gen_css_buttons.py` for reproducible CSS button data generation.

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
