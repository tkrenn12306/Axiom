mod commands;
mod config;
mod repl;

use axiom_core::TickEngine;
use tracing_subscriber::EnvFilter;

fn main() {
    // Initialize CLI configuration
    let _config = config::CliConfig::load();

    // Initialize structured logging (respects RUST_LOG env var)
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("load") => {
            let path = args.get(2).expect("Usage: axiom load <file.ax>");
            commands::load_file(path);
        }
        Some("inspect") => {
            let target = args.get(2).expect("Usage: axiom inspect <entity.Name>");
            commands::inspect(target);
        }
        Some("repl") | None => {
            repl::start();
        }
        Some(cmd) => {
            eprintln!("Unknown command: {}", cmd);
            eprintln!("Usage: axiom [load <file.ax> | inspect <entity.Name> | repl]");
            std::process::exit(1);
        }
    }
}

/// Phase 0 deliverable: run the tick engine standalone.
/// Called internally by `repl::start()` when no .ax file is loaded.
pub fn run_tick_engine() {
    let mut engine = TickEngine::new(60);
    engine.run();
}
