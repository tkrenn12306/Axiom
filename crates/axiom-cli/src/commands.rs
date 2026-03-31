use std::collections::HashMap;

use axiom_core::{EntityId, World};
use axiom_lang::{
    evaluator::{LoadResult, Mass, Position, Temperature, Velocity, Height, Properties, EntityTypeTag},
    parse_file, load_into_world,
};

/// Shared simulation state passed around in the REPL session.
pub struct SimState {
    pub world: World,
    pub named_entities: HashMap<String, EntityId>,
    pub world_name: Option<String>,
}

impl SimState {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            named_entities: HashMap::new(),
            world_name: None,
        }
    }
}

impl Default for SimState {
    fn default() -> Self {
        Self::new()
    }
}

/// `axiom load <path>` — parse an `.ax` file and load it into a fresh world.
/// Prints entity count and world name.
pub fn load_file(path: &str) {
    let mut state = SimState::new();
    match do_load(path, &mut state) {
        Ok(result) => {
            if let Some(name) = &result.world_name {
                println!("World: \"{}\"", name);
            }
            println!("Loaded {} entities from '{}'", result.entity_count, path);
            for name in result.named_entities.keys() {
                println!("  - {}", name);
            }
        }
        Err(e) => {
            eprintln!("Error loading '{}': {}", path, e);
            std::process::exit(1);
        }
    }
}

/// `axiom inspect <entity.Name>` — inspect an entity by dotted name.
/// The format is `entity.<Name>` or just `<Name>`.
pub fn inspect(target: &str) {
    // We need a loaded world to inspect. For standalone invocation, note
    // the limitation and suggest using the REPL.
    eprintln!(
        "To inspect entities, use the REPL:\n  axiom repl\n  > load <file.ax>\n  > inspect {}",
        target
    );
    std::process::exit(1);
}

/// Load a file into an existing SimState, returns LoadResult or error string.
pub fn do_load(path: &str, state: &mut SimState) -> Result<LoadResult, String> {
    let src = std::fs::read_to_string(path)
        .map_err(|e| format!("Cannot read file: {}", e))?;

    let file = parse_file(&src)
        .map_err(|e| format!("{}", e))?;

    // Reset world for fresh load
    state.world = axiom_core::World::new();
    let result = load_into_world(file, &mut state.world);

    state.named_entities = result.named_entities.clone();
    state.world_name = result.world_name.clone();

    Ok(result)
}

/// Inspect an entity by name in the current SimState.
/// Returns a formatted multi-line string of the entity's components.
pub fn do_inspect(name: &str, state: &SimState) -> Result<String, String> {
    // Accept both "entity.Elena" and "Elena"
    let lookup_name = name.strip_prefix("entity.").unwrap_or(name);

    let entity_id = state
        .named_entities
        .get(lookup_name)
        .copied()
        .ok_or_else(|| format!("Entity '{}' not found", lookup_name))?;

    let mut lines = Vec::new();
    lines.push(format!("Entity: {}", lookup_name));
    lines.push("─".repeat(40));

    if let Ok(pos) = state.world.get::<Position>(entity_id) {
        lines.push(format!("  position:     ({:.2}, {:.2}) m", pos.x, pos.y));
    }
    if let Ok(mass) = state.world.get::<Mass>(entity_id) {
        lines.push(format!("  mass:         {:.2} kg", mass.0));
    }
    if let Ok(height) = state.world.get::<Height>(entity_id) {
        lines.push(format!("  height:       {:.2} m", height.0));
    }
    if let Ok(temp) = state.world.get::<Temperature>(entity_id) {
        // Show in Celsius for readability
        let celsius = temp.0 - 273.15;
        lines.push(format!("  temperature:  {:.2} °C ({:.2} K)", celsius, temp.0));
    }
    if let Ok(vel) = state.world.get::<Velocity>(entity_id) {
        lines.push(format!("  velocity:     ({:.3}, {:.3}) m/s", vel.vx, vel.vy));
    }
    if let Ok(tag) = state.world.get::<EntityTypeTag>(entity_id) {
        lines.push(format!("  type:         {:?}", tag.0));
    }
    if let Ok(props) = state.world.get::<Properties>(entity_id) {
        lines.push(format!("  properties:"));
        let mut sorted_keys: Vec<&String> = props.0.keys().collect();
        sorted_keys.sort();
        for key in sorted_keys {
            // Skip the keys we already showed above
            if matches!(key.as_str(), "type") {
                continue;
            }
            lines.push(format!("    {}: {}", key, props.0[key]));
        }
    }

    Ok(lines.join("\n"))
}
