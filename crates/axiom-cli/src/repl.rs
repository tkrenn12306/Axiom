use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Context;
use rustyline::DefaultEditor;

use crate::commands::{do_inspect, do_load, SimState};
use axiom_core::TickEngine;

const BANNER: &str = r#"
  █████╗ ██╗  ██╗██╗ ██████╗ ███╗   ███╗
 ██╔══██╗╚██╗██╔╝██║██╔═══██╗████╗ ████║
 ███████║ ╚███╔╝ ██║██║   ██║██╔████╔██║
 ██╔══██║ ██╔██╗ ██║██║   ██║██║╚██╔╝██║
 ██║  ██║██╔╝ ██╗██║╚██████╔╝██║ ╚═╝ ██║
 ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═╝
  Terminal Physics Simulation — v0.1.0-alpha
  Type 'help' for available commands.
"#;

const HELP: &str = r#"
Available commands:
  load <file.ax>            Load and parse an AxiomLang simulation file
  inspect <entity.Name>     Show all components for an entity
  inspect entity.<Name>     Same as above (dotted form)
  list                      List all spawned entities
  world                     Show world configuration
  run [ticks]               Run the tick engine (Ctrl-C to stop, or N ticks)
  time.pause                Pause the simulation
  time.resume               Resume the simulation
  time.step [N]             Advance N ticks (default: 1)
  time.speed <factor>       Set simulation speed multiplier (e.g., 2x, 0.5x)
  clear                     Clear the screen
  help                      Show this help
  quit / exit               Exit the REPL
"#;

/// Custom completer for Axiom commands and entity names.
struct AxiomCompleter {
    file_completer: FilenameCompleter,
}

impl AxiomCompleter {
    fn new() -> Self {
        Self {
            file_completer: FilenameCompleter::new(),
        }
    }

    fn get_base_commands(&self) -> Vec<&'static str> {
        vec![
            "load",
            "inspect",
            "list",
            "world",
            "run",
            "clear",
            "help",
            "quit",
            "exit",
            "time.pause",
            "time.resume",
            "time.step",
            "time.speed",
        ]
    }
}

impl Completer for AxiomCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let line = &line[..pos];
        let parts: Vec<&str> = line.split_whitespace().collect();

        // If at start of line, complete command names
        if parts.is_empty() || (parts.len() == 1 && !line.ends_with(' ')) {
            let partial = parts.first().copied().unwrap_or("");
            let commands = self.get_base_commands();
            let candidates: Vec<Pair> = commands
                .iter()
                .filter(|cmd| cmd.starts_with(partial))
                .map(|cmd| Pair {
                    display: cmd.to_string(),
                    replacement: cmd.to_string(),
                })
                .collect();
            return Ok((0, candidates));
        }

        // For file paths after "load", use filename completion
        if let Some(cmd) = parts.first() {
            if *cmd == "load" && parts.len() >= 2 {
                // Simplified file completion - just return empty
                return Ok((0, Vec::new()));
            }
        }

        Ok((0, Vec::new()))
    }
}

impl Hinter for AxiomCompleter {
    type Hint = String;
}

impl Highlighter for AxiomCompleter {}

impl Validator for AxiomCompleter {}

pub fn start() {
    println!("{}", BANNER);

    let mut rl = DefaultEditor::new().expect("Failed to initialize readline");
    rl.set_max_history_size(1000).ok();

    let mut state = SimState::new();

    loop {
        let readline = rl.readline("axiom> ");
        match readline {
            Ok(line) => {
                let line = line.trim().to_string();
                if line.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(line.as_str());
                handle_command(&line, &mut state);
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

fn handle_command(line: &str, state: &mut SimState) {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    let cmd = parts[0];
    let arg = parts.get(1).copied().unwrap_or("").trim();

    match cmd {
        "help" | "?" => println!("{}", HELP),
        "clear" => {
            print!("\x1B[2J\x1B[H");
        }
        "quit" | "exit" | "q" => {
            println!("Goodbye.");
            std::process::exit(0);
        }
        "load" => {
            if arg.is_empty() {
                eprintln!("Usage: load <file.ax>");
                return;
            }
            match do_load(arg, state) {
                Ok(result) => {
                    if let Some(name) = &result.world_name {
                        println!("World: \"{}\"", name);
                    }
                    println!("Loaded {} entities from '{}'", result.entity_count, arg);
                    for name in result.named_entities.keys() {
                        println!("  - {}", name);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "inspect" => {
            if arg.is_empty() {
                eprintln!("Usage: inspect <entity.Name>");
                return;
            }
            match do_inspect(arg, state) {
                Ok(output) => println!("{}", output),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "list" => {
            if state.named_entities.is_empty() {
                println!("No entities loaded. Use 'load <file.ax>' first.");
            } else {
                println!("Entities ({}):", state.named_entities.len());
                let mut names: Vec<&String> = state.named_entities.keys().collect();
                names.sort();
                for name in names {
                    println!("  - {}", name);
                }
            }
        }
        "world" => match &state.world_name {
            Some(name) => println!("World: \"{}\"  |  Entities: {}", name, state.world.len()),
            None => println!("No world loaded. Use 'load <file.ax>' first."),
        },
        "run" => {
            if arg.is_empty() {
                // Run tick engine indefinitely (Phase 0 deliverable)
                let mut engine = TickEngine::new(60);
                engine.run();
            } else if let Ok(n) = arg.parse::<u64>() {
                let mut engine = TickEngine::new(60);
                engine.run_n(n);
                println!("Ran {} ticks.", n);
            } else {
                eprintln!("Usage: run [N]   (N = number of ticks, omit for infinite)");
            }
        }
        "time.pause" => {
            println!("⏸  Simulation paused (Not yet implemented in Phase 2)");
        }
        "time.resume" => {
            println!("▶  Simulation resumed (Not yet implemented in Phase 2)");
        }
        "time.step" => {
            if arg.is_empty() {
                println!("Stepped 1 tick (Not yet fully implemented in Phase 2)");
            } else if let Ok(n) = arg.parse::<u64>() {
                println!("Stepped {} ticks (Not yet fully implemented in Phase 2)", n);
            } else {
                eprintln!("Usage: time.step [N]   (N = number of ticks, default: 1)");
            }
        }
        "time.speed" => {
            if arg.is_empty() {
                eprintln!("Usage: time.speed <factor>   (e.g., 2x for double speed)");
            } else {
                let factor_str = arg.strip_suffix('x').unwrap_or(arg);
                match factor_str.parse::<f64>() {
                    Ok(factor) => println!(
                        "⏱  Simulation speed set to {}x (Not yet implemented in Phase 2)",
                        factor
                    ),
                    Err(_) => eprintln!("Invalid speed factor. Use format like '2x' or '0.5x'"),
                }
            }
        }
        _ => {
            eprintln!(
                "Unknown command: '{}'. Type 'help' for available commands.",
                cmd
            );
        }
    }
}
