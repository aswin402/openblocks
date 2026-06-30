use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, EnvFilter};

mod config;
mod db;
mod error;
mod models;
mod search;
mod server;

#[derive(Parser)]
#[command(name = "openblocks")]
#[command(about = "A Rust-native MCP server for web development")]
#[command(version)]
struct Cli {
    /// Path to SQLite database
    #[arg(long, env = "OPENBLOCKS_DB_PATH")]
    db_path: Option<String>,

    /// Log level (error, warn, info, debug, trace)
    #[arg(long, default_value = "info", env = "OPENBLOCKS_LOG_LEVEL")]
    log_level: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the MCP server (default)
    Serve,
    /// Seed the database with starter components
    Seed,
    /// Show library statistics
    Stats,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // CRITICAL: Log to stderr ONLY — stdout is the MCP JSON-RPC channel
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&cli.log_level));

    fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)  // NEVER use stdout
        .init();

    tracing::info!("OpenBlocks v{}", env!("CARGO_PKG_VERSION"));

    // Determine database path
    let db_path = cli.db_path.unwrap_or_else(|| {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("openblocks");
        std::fs::create_dir_all(&data_dir).ok();
        data_dir.join("openblocks.db").to_string_lossy().to_string()
    });

    tracing::info!("Database: {}", db_path);

    // Initialize database
    let mut db = db::Database::new(&db_path)?;
    db.run_migrations()?;

    // Build search index
    let components = db.list_all_components()?;
    let mut search_engine = search::SearchEngine::new();
    search_engine.rebuild(&components);

    tracing::info!("Loaded {} components into search index", components.len());

    match cli.command.unwrap_or(Commands::Serve) {
        Commands::Serve => {
            tracing::info!("Starting MCP server on stdio transport...");
            let server = server::OpenBlocksServer::new(db, search_engine);
            server.serve().await?;
        }
        Commands::Seed => {
            tracing::info!("Seeding database with starter components...");
            let count = db.seed_from_file("data/seed_components.json")?;
            eprintln!("Seeded {} components", count);
        }
        Commands::Stats => {
            let stats = db.get_stats()?;
            eprintln!("OpenBlocks Library Statistics:");
            eprintln!("  Components: {}", stats.total_components);
            eprintln!("  Templates:  {}", stats.total_templates);
            eprintln!("  Categories: {}", stats.categories.len());
            eprintln!("  Frameworks: {}", stats.frameworks.len());
        }
    }

    Ok(())
}
