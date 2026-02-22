mod cli;
mod core;
mod language;
mod template;
mod tui;

fn main() -> anyhow::Result<()> {
    cli::run()
}
