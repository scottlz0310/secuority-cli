use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "secuority", version, about = "Multi-language security & quality configuration tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Skip interactive TUI and apply all configurations automatically
    #[arg(long, global = true)]
    non_interactive: bool,

    /// Path to the target project (default: current directory)
    #[arg(short, long, global = true, default_value = ".")]
    project_path: std::path::PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze project and show recommendations
    Check,
    /// Apply security & quality configurations interactively
    Apply,
    /// Update templates from remote
    Update,
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Check => {
            println!("check: not yet implemented");
        }
        Commands::Apply => {
            println!("apply: not yet implemented");
        }
        Commands::Update => {
            println!("update: not yet implemented");
        }
    }
    Ok(())
}
