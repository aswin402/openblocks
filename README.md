# 🧱 OpenBlocks

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)
[![MCP](https://img.shields.io/badge/mcp-protocol-blue.svg)](https://modelcontextprotocol.io)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**OpenBlocks** is a fast, local-first, Rust-native Model Context Protocol (MCP) server designed specifically for web development. It enables AI coding agents (like Claude Desktop, Cursor, Cline, or Roo Code) to query, retrieve, write, and modify pre-built UI components and website templates rather than building them from scratch.

By providing a structured interface for UI building blocks, OpenBlocks significantly reduces the time and context-window token overhead consumed by LLMs during frontend generation.

---

## 🚀 Core Features

- **Component Registry**: Store and manage modular UI components (HTML, CSS, SCSS, Tailwind, Shadcn UI, React).
- **Fuzzy Search Engine**: Local fuzzy indexing of component names, descriptions, and tags.
- **Template System**: Combine components into full website templates and scaffold complete pages using MiniJinja rendering.
- **Version Control**: Auto-track history of modified components.
- **Local-First & Fast**: Built on SQLite (WAL mode) with search execution sub-millisecond response times.
- **Import/Export**: Easy JSON backup/restore and database seeding.

---

## 🛠️ Getting Started

### 1. Build from Source
Ensure you have Rust and Cargo installed:
```bash
# Clone the repository
git clone https://github.com/yourusername/openblocks.git
cd openblocks

# Build release binary
cargo build --release
```

### 2. Seed Starter Components
Populate your database with the built-in starter blocks (Navbar, Hero, Footer, etc.):
```bash
cargo run --release -- seed
```

### 3. Verify Stats
```bash
cargo run --release -- stats
```

---

## 💻 CLI Subcommands

OpenBlocks can be run in different modes:
- `serve` (Default): Starts the MCP server on stdio transport.
- `seed`: Seeds the database with starter components from data files (`seed_components.json`, `uiverse_components.json`, `tailblocks_components.json`, `css_text_animations.json`, `heroui_components.json`, `css_buttons.json`) plus built-in palettes and gradients.
- `stats`: Displays database statistics (component counts, categories, frameworks).

Flags:
- `--db-path <PATH>`: Custom path to SQLite database.
- `--log-level <LEVEL>`: Set log level (error, warn, info, debug, trace). All logging goes to `stderr` to avoid corrupting the MCP communication stream.

---

## 🧩 MCP Tools Reference

OpenBlocks exposes 15 tools to connected AI agents:

### Component Discovery & Retrieval
1. `search_components`: Fuzzy search against component metadata (name, description, tags).
2. `get_component`: Retrieve the full source code and dependencies of a component by ID.
3. `list_categories`: View all categories (navbar, hero, footer, card, etc.) with counts.
4. `list_frameworks`: View supported frameworks (tailwind, css, scss, shadcn, react) with counts.
5. `get_stats`: Retrieve library-wide stats.

### Component CRUD (Mutations)
6. `add_component`: Add a new UI block to the library.
7. `update_component`: Update an existing component and auto-increment its version history.
8. `delete_component`: Delete a component permanently by ID.

### Templates & Scaffolding
9. `list_templates`: List all available layout templates.
10. `get_template`: Retrieve a specific layout template by ID.
11. `add_template`: Add a new website template layout.
12. `delete_template`: Delete a template layout by ID.
13. `scaffold_page`: Compile and render complete HTML pages from templates with variable injection.

### Import / Export
14. `import_components`: Batch import components from a local JSON file.
15. `export_components`: Export library components to a local JSON file.

---

## ⚙️ Configuration for AI Clients

Add the following to your AI client configuration files to integrate OpenBlocks.

### Claude Desktop
Add to `~/.config/Claude/claude_desktop_config.json`:
```json
{
  "mcpServers": {
    "openblocks": {
      "command": "cargo",
      "args": ["run", "--manifest-path", "/path/to/openblocks/Cargo.toml", "--", "serve"]
    }
  }
}
```

### Cursor IDE
Go to **Settings > Features > MCP**, click **+ Add New MCP Server**:
- **Name**: `openblocks`
- **Type**: `command`
- **Command**: `cargo run --manifest-path /path/to/openblocks/Cargo.toml -- serve`

---

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
