use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenBlocksError {
    #[error("Component not found: {0}")]
    ComponentNotFound(String),

    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Palette not found: {0}")]
    PaletteNotFound(String),

    #[error("Invalid category: '{0}'. Valid: navbar, hero, footer, sidebar, card, form, modal, table, pricing, testimonial, cta, feature, faq, contact, auth, dashboard, settings, profile, landing, blog, ecommerce, error, loading, notification, section, other")]
    InvalidCategory(String),

    #[error("Invalid framework: '{0}'. Valid: tailwind, css, scss, shadcn")]
    InvalidFramework(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Import error: {0}")]
    Import(String),

    #[error("Template render error: {0}")]
    Render(String),
}

pub type Result<T> = std::result::Result<T, OpenBlocksError>;
