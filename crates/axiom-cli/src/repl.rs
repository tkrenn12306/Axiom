use rustyline::error::ReadlineError;
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
  clear                     Clear the screen
  help                      Show this help
  quit / exit               Exit the REPL
"#;

/// Start the interactive REPL.
pub fn start() {
    println!("{}", BANNER);

    let mut rl = DefaultEditor::new().expect("Failed to initialize readline");
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
        "world" => {
            match &state.world_name {
                Some(name) => println!("World: \"{}\"  |  Entities: {}", name, state.world.len()),
                None => println!("No world loaded. Use 'load <file.ax>' first."),
            }
        }
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
        _ => {
            eprintln!("Unknown command: '{}'. Type 'help' for available commands.", cmd);
        }
    }
}
