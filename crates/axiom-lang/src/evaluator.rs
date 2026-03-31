use std::collections::HashMap;

use crate::ast::{AxiomFile, EntityDef, EntityType, Value};
use axiom_core::{EntityId, World};

/// ECS components added by the evaluator.

/// The entity's name in the simulation (e.g. "Elena").
#[derive(Debug, Clone)]
pub struct Name(pub String);

/// 2D position on the world grid (meters).
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

/// Mass in kilograms.
#[derive(Debug, Clone, Copy)]
pub struct Mass(pub f64);

/// Height in meters.
#[derive(Debug, Clone, Copy)]
pub struct Height(pub f64);

/// Body temperature in Kelvin.
#[derive(Debug, Clone, Copy)]
pub struct Temperature(pub f64);

/// Velocity vector in m/s.
#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity {
    pub vx: f64,
    pub vy: f64,
}

/// Tag component for the entity type.
#[derive(Debug, Clone)]
pub struct EntityTypeTag(pub EntityType);

/// Stores all raw property key-value pairs for inspection.
#[derive(Debug, Clone)]
pub struct Properties(pub HashMap<String, Value>);

/// Result of loading an `.ax` file into the ECS world.
pub struct LoadResult {
    /// Maps instance name → EntityId for `inspect` lookups.
    pub named_entities: HashMap<String, EntityId>,
    /// Total number of entities spawned.
    pub entity_count: usize,
    /// Name of the loaded world (if defined).
    pub world_name: Option<String>,
}

/// Load a parsed `AxiomFile` into the ECS `World`.
///
/// For each `spawn` command, looks up the corresponding entity definition,
/// applies instance overrides, and creates ECS entities with appropriate components.
pub fn load_into_world(file: AxiomFile, world: &mut World) -> LoadResult {
    let mut named_entities = HashMap::new();

    // Index entity definitions by name for quick lookup
    let entity_defs: HashMap<String, &EntityDef> =
        file.entities.iter().map(|e| (e.name.clone(), e)).collect();

    for cmd in &file.spawn_cmds {
        // Merge definition props with instance overrides
        let mut merged_props: HashMap<String, Value> = entity_defs
            .get(&cmd.entity_def)
            .map(|def| def.props.clone())
            .unwrap_or_default();

        for (k, v) in &cmd.overrides {
            merged_props.insert(k.clone(), v.clone());
        }

        let entity_type = entity_defs
            .get(&cmd.entity_def)
            .map(|def| def.entity_type.clone())
            .unwrap_or(EntityType::Generic);

        // Extract standard physics components
        let mass_kg = merged_props
            .get("mass")
            .and_then(|v| v.as_si())
            .unwrap_or(1.0);
        let height_m = merged_props
            .get("height")
            .and_then(|v| v.as_si())
            .unwrap_or(1.0);
        let temp_k = merged_props
            .get("body_temperature")
            .and_then(|v| v.as_si())
            .unwrap_or(293.15); // 20°C default

        let position = cmd
            .position
            .map(|(x, y)| Position { x, y })
            .unwrap_or(Position { x: 0.0, y: 0.0 });

        let entity_id = world.spawn((
            Name(cmd.instance_name.clone()),
            position,
            Mass(mass_kg),
            Height(height_m),
            Temperature(temp_k),
            Velocity::default(),
            EntityTypeTag(entity_type),
            Properties(merged_props),
        ));

        named_entities.insert(cmd.instance_name.clone(), entity_id);
    }

    let entity_count = named_entities.len();
    let world_name = file.world.as_ref().map(|w| w.name.clone());

    LoadResult {
        named_entities,
        entity_count,
        world_name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_file;

    fn make_test_src() -> &'static str {
        r#"
entity person {
    type: humanoid
    mass: 75
    height: 1.78
    body_temperature: 37
}

spawn person "Elena" at (120, 200) {
    mass: 62
}

spawn person "Bob" at (50, 50)
"#
    }

    #[test]
    fn test_load_entities() {
        let file = parse_file(make_test_src()).unwrap();
        let mut world = World::new();
        let result = load_into_world(file, &mut world);

        assert_eq!(result.entity_count, 2);
        assert!(result.named_entities.contains_key("Elena"));
        assert!(result.named_entities.contains_key("Bob"));
    }

    #[test]
    fn test_instance_override_applied() {
        let file = parse_file(make_test_src()).unwrap();
        let mut world = World::new();
        let result = load_into_world(file, &mut world);

        let elena_id = result.named_entities["Elena"];
        let mass = world.get::<Mass>(elena_id).unwrap();
        // Elena overrides mass to 62 kg
        assert!((mass.0 - 62.0).abs() < 1e-6);
    }

    #[test]
    fn test_position_set() {
        let file = parse_file(make_test_src()).unwrap();
        let mut world = World::new();
        let result = load_into_world(file, &mut world);

        let elena_id = result.named_entities["Elena"];
        let pos = world.get::<Position>(elena_id).unwrap();
        assert!((pos.x - 120.0).abs() < 1e-6);
        assert!((pos.y - 200.0).abs() < 1e-6);
    }

    #[test]
    fn test_world_entity_count() {
        let file = parse_file(make_test_src()).unwrap();
        let mut world = World::new();
        load_into_world(file, &mut world);
        assert_eq!(world.len(), 2);
    }
}
