use crate::db::Database;
use crate::models::component::{NewComponent, UpdateComponent};
use crate::models::gradient::NewGradient;
use crate::models::palette::NewPalette;
use crate::models::template::NewTemplate;
use crate::search::SearchEngine;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, Content, ErrorData};
use rmcp::{ServerHandler, tool, tool_handler, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

/// Shared state wrapped for cloning and interior mutability
#[derive(Clone)]
pub struct OpenBlocksServer {
    inner: Arc<OpenBlocksServerInner>,
    tool_router: rmcp::handler::server::tool::ToolRouter<Self>,
}

pub struct OpenBlocksServerInner {
    db: Mutex<Database>,
    search: Mutex<SearchEngine>,
}

impl OpenBlocksServer {
    pub fn new(db: Database, search: SearchEngine) -> Self {
        Self {
            inner: Arc::new(OpenBlocksServerInner {
                db: Mutex::new(db),
                search: Mutex::new(search),
            }),
            tool_router: Self::tool_router(),
        }
    }

    pub async fn serve(self) -> anyhow::Result<()> {
        // Serve using stdio transport
        rmcp::ServiceExt::serve(self, rmcp::transport::stdio()).await?;
        Ok(())
    }
}

// --- Input structs for MCP tools ---

#[derive(Debug, Deserialize, JsonSchema)]
struct SearchInput {
    /// Search text (fuzzy matched against names, descriptions, and tags)
    query: String,
    /// Filter by category: navbar, hero, footer, sidebar, card, form, modal, etc.
    category: Option<String>,
    /// Filter by framework: tailwind, css, scss, shadcn
    framework: Option<String>,
    /// Maximum results to return (default: 10)
    limit: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct GetInput {
    /// Component UUID
    id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct DeleteInput {
    /// Component UUID to delete
    id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct GetTemplateInput {
    /// Template UUID
    id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct DeleteTemplateInput {
    /// Template UUID to delete
    id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ScaffoldInput {
    /// Template UUID to scaffold from
    template_id: String,
    /// Variables to pass to layout rendering (JSON object)
    #[serde(default)]
    variables: serde_json::Value,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ImportInput {
    /// Path to JSON file containing components to import
    file_path: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ExportInput {
    /// Output path for the exported JSON file
    output_path: String,
    /// Filter by category (optional)
    category: Option<String>,
    /// Filter by framework (optional)
    framework: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct GetPaletteInput {
    /// Palette UUID
    id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct DeletePaletteInput {
    /// Palette UUID to delete
    id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct GetGradientInput {
    /// Gradient UUID
    id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct DeleteGradientInput {
    /// Gradient UUID to delete
    id: String,
}

// --- MCP Tool Router ---

#[tool_router]
impl OpenBlocksServer {
    #[tool(
        description = "Search the UI component library. Fuzzy matches against component names, descriptions, and tags. Optionally filter by category (navbar, hero, footer, card, form, modal, pricing, etc.) and framework (tailwind, css, scss, shadcn). Returns component metadata (no code) — use get_component to retrieve code."
    )]
    async fn search_components(
        &self,
        Parameters(input): Parameters<SearchInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let limit = input.limit.unwrap_or(10);

        let search = self.inner.search.lock().unwrap();
        let matching_ids = search.search(&input.query);
        drop(search);

        let db = self.inner.db.lock().unwrap();
        let mut results = Vec::new();

        for id in matching_ids.into_iter() {
            if let Ok(component) = db.get_component(&id.to_string()) {
                // Apply filters
                if let Some(ref cat) = input.category
                    && component.category.to_string() != *cat
                {
                    continue;
                }
                if let Some(ref fw) = input.framework
                    && component.framework.to_string() != *fw
                {
                    continue;
                }
                results.push(serde_json::json!({
                    "id": component.id,
                    "name": component.name,
                    "description": component.description,
                    "category": component.category,
                    "framework": component.framework,
                    "tags": component.tags,
                    "version": component.version,
                }));
                if results.len() >= limit {
                    break;
                }
            }
        }

        let response = serde_json::json!({
            "results": results,
            "total": results.len(),
            "query": input.query,
        });

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(
        description = "Get the full details and source code of a component by its ID. Use this after search_components to retrieve the actual code."
    )]
    async fn get_component(
        &self,
        Parameters(input): Parameters<GetInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.get_component(&input.id) {
            Ok(component) => {
                let json = serde_json::to_string_pretty(&component).unwrap();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(
        description = "Add a new UI component to the library. Provide name, description, category (navbar/hero/footer/card/form/modal/pricing/etc.), framework (tailwind/css/scss/shadcn), the HTML/CSS/JS code, and searchable tags."
    )]
    async fn add_component(
        &self,
        Parameters(input): Parameters<NewComponent>,
    ) -> Result<CallToolResult, ErrorData> {
        // Validate input
        if let Err(e) = input.validate() {
            return Ok(CallToolResult::error(vec![Content::text(e.to_string())]));
        }

        let db = self.inner.db.lock().unwrap();
        match db.insert_component(&input) {
            Ok(component) => {
                // Update search index
                drop(db);
                let mut search = self.inner.search.lock().unwrap();
                search.index_component(&component);

                let response = serde_json::json!({
                    "id": component.id,
                    "name": component.name,
                    "version": component.version,
                    "message": "Component added successfully"
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(
        description = "Update an existing component. Provide the component ID and any fields to change (name, description, code, tags, category, framework). Unchanged fields keep their current values. Creates a version history entry."
    )]
    async fn update_component(
        &self,
        Parameters(input): Parameters<UpdateComponent>,
    ) -> Result<CallToolResult, ErrorData> {
        if let Err(e) = input.validate() {
            return Ok(CallToolResult::error(vec![Content::text(e.to_string())]));
        }

        let db = self.inner.db.lock().unwrap();
        match db.update_component(&input) {
            Ok(component) => {
                // Rebuild search index for this component
                drop(db);
                let db = self.inner.db.lock().unwrap();
                let all = db.list_all_components().unwrap_or_default();
                drop(db);
                let mut search = self.inner.search.lock().unwrap();
                search.rebuild(&all);

                let response = serde_json::json!({
                    "id": component.id,
                    "name": component.name,
                    "version": component.version,
                    "message": "Component updated successfully"
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(
        description = "Delete a component from the library by its ID. This action is permanent."
    )]
    async fn delete_component(
        &self,
        Parameters(input): Parameters<DeleteInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.delete_component(&input.id) {
            Ok(()) => {
                // Rebuild search index
                let all = db.list_all_components().unwrap_or_default();
                drop(db);
                let mut search = self.inner.search.lock().unwrap();
                search.rebuild(&all);

                let response = serde_json::json!({
                    "id": input.id,
                    "message": "Component deleted successfully"
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(
        description = "List all available component categories with the count of components in each."
    )]
    async fn list_categories(&self) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.get_category_counts() {
            Ok(counts) => {
                let json = serde_json::to_string_pretty(&counts).unwrap();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "List all supported CSS frameworks with the count of components in each.")]
    async fn list_frameworks(&self) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.get_framework_counts() {
            Ok(counts) => {
                let json = serde_json::to_string_pretty(&counts).unwrap();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(
        description = "Get library-wide statistics: total components, total templates, category breakdown, framework breakdown."
    )]
    async fn get_stats(&self) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.get_stats() {
            Ok(stats) => {
                let json = serde_json::to_string_pretty(&stats).unwrap();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    // --- Templates Tools ---

    #[tool(description = "List all available website templates.")]
    async fn list_templates(&self) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.list_templates() {
            Ok(templates) => {
                let json = serde_json::to_string_pretty(&templates).unwrap();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Get a single template by its ID.")]
    async fn get_template(
        &self,
        Parameters(input): Parameters<GetTemplateInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.get_template(&input.id) {
            Ok(template) => {
                let json = serde_json::to_string_pretty(&template).unwrap();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(
        description = "Add a new website template. Provide name, description, component_ids (ordered list of component UUIDs), layout (JSON object specifying base template/structure), and variables (default parameters)."
    )]
    async fn add_template(
        &self,
        Parameters(input): Parameters<NewTemplate>,
    ) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.insert_template(&input) {
            Ok(template) => {
                let response = serde_json::json!({
                    "id": template.id,
                    "name": template.name,
                    "message": "Template added successfully"
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Delete a template by its ID.")]
    async fn delete_template(
        &self,
        Parameters(input): Parameters<DeleteTemplateInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.delete_template(&input.id) {
            Ok(()) => {
                let response = serde_json::json!({
                    "id": input.id,
                    "message": "Template deleted successfully"
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(
        description = "Scaffold a complete website page from a template ID, substituting customizable template variables."
    )]
    async fn scaffold_page(
        &self,
        Parameters(input): Parameters<ScaffoldInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.scaffold_page(&input.template_id, &input.variables) {
            Ok(html) => Ok(CallToolResult::success(vec![Content::text(html)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    // --- Import / Export Tools ---

    #[tool(
        description = "Import components from a local JSON file path. The file must contain a JSON array of components."
    )]
    async fn import_components(
        &self,
        Parameters(input): Parameters<ImportInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.seed_from_file(&input.file_path) {
            Ok(count) => {
                // Rebuild search index
                let all = db.list_all_components().unwrap_or_default();
                drop(db);
                let mut search = self.inner.search.lock().unwrap();
                search.rebuild(&all);

                let response = serde_json::json!({
                    "count": count,
                    "message": format!("Successfully imported {} components", count)
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(
        description = "Export components to a local JSON file path. Optionally filter by category and/or framework."
    )]
    async fn export_components(
        &self,
        Parameters(input): Parameters<ExportInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        let category = input.category.as_deref();
        let framework = input.framework.as_deref();
        match db.list_components(category, framework, usize::MAX) {
            Ok(components) => {
                drop(db);
                match serde_json::to_string_pretty(&components) {
                    Ok(json) => {
                        if let Err(e) = std::fs::write(&input.output_path, json) {
                            return Ok(CallToolResult::error(vec![Content::text(format!(
                                "Failed to write file: {}",
                                e
                            ))]));
                        }
                        let response = serde_json::json!({
                            "count": components.len(),
                            "file_path": input.output_path,
                            "message": format!("Successfully exported {} components", components.len())
                        });
                        Ok(CallToolResult::success(vec![Content::text(
                            serde_json::to_string_pretty(&response).unwrap(),
                        )]))
                    }
                    Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                }
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    // --- Color Palette Tools (Inspired by Color Hunt) ---

    #[tool(description = "List all available color palettes stored in the library.")]
    async fn list_palettes(&self) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.list_palettes() {
            Ok(palettes) => {
                let json = serde_json::to_string_pretty(&palettes).unwrap();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Get details of a single color palette by its ID.")]
    async fn get_palette(
        &self,
        Parameters(input): Parameters<GetPaletteInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.get_palette(&input.id) {
            Ok(palette) => {
                let json = serde_json::to_string_pretty(&palette).unwrap();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(
        description = "Add a new color palette to the library. Provide a name, exactly 4 hex color codes, and descriptive tags (e.g. ['dark', 'pastel', 'warm', 'nordic'])."
    )]
    async fn add_palette(
        &self,
        Parameters(input): Parameters<NewPalette>,
    ) -> Result<CallToolResult, ErrorData> {
        if let Err(e) = input.validate() {
            return Ok(CallToolResult::error(vec![Content::text(e.to_string())]));
        }
        let db = self.inner.db.lock().unwrap();
        match db.insert_palette(&input) {
            Ok(palette) => {
                let response = serde_json::json!({
                    "id": palette.id,
                    "name": palette.name,
                    "message": "Palette added successfully"
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Delete a color palette from the library by its ID.")]
    async fn delete_palette(
        &self,
        Parameters(input): Parameters<DeletePaletteInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.delete_palette(&input.id) {
            Ok(()) => {
                let response = serde_json::json!({
                    "id": input.id,
                    "message": "Palette deleted successfully"
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    // --- Color Gradient Tools (Inspired by Coolors Gradients) ---

    #[tool(description = "List all available color gradients stored in the library.")]
    async fn list_gradients(&self) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.list_gradients() {
            Ok(gradients) => {
                let json = serde_json::to_string_pretty(&gradients).unwrap();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Get details of a single color gradient by its ID.")]
    async fn get_gradient(
        &self,
        Parameters(input): Parameters<GetGradientInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.get_gradient(&input.id) {
            Ok(gradient) => {
                let json = serde_json::to_string_pretty(&gradient).unwrap();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(
        description = "Add a new color gradient to the library. Provide a name, CSS description string (e.g. 'linear-gradient(90deg, #ff007f 0%, #7f00ff 100%)'), and list of hex color codes."
    )]
    async fn add_gradient(
        &self,
        Parameters(input): Parameters<NewGradient>,
    ) -> Result<CallToolResult, ErrorData> {
        if let Err(e) = input.validate() {
            return Ok(CallToolResult::error(vec![Content::text(e.to_string())]));
        }
        let db = self.inner.db.lock().unwrap();
        match db.insert_gradient(&input) {
            Ok(gradient) => {
                let response = serde_json::json!({
                    "id": gradient.id,
                    "name": gradient.name,
                    "message": "Gradient added successfully"
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Delete a color gradient from the library by its ID.")]
    async fn delete_gradient(
        &self,
        Parameters(input): Parameters<DeleteGradientInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let db = self.inner.db.lock().unwrap();
        match db.delete_gradient(&input.id) {
            Ok(()) => {
                let response = serde_json::json!({
                    "id": input.id,
                    "message": "Gradient deleted successfully"
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }
}

#[tool_handler]
impl ServerHandler for OpenBlocksServer {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo {
            server_info: rmcp::model::Implementation {
                name: "openblocks".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
