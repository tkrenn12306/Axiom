// Integration test: parse and load scenarios/test_basic.ax into the ECS world.

use axiom_core::World;
use axiom_lang::{load_into_world, parse_file};

#[test]
fn test_load_test_basic_scenario() {
    let src = include_str!("../../scenarios/test_basic.ax");
    let file = parse_file(src).expect("Should parse without errors");

    // World block should be present
    let world_cfg = file.world.as_ref().expect("Should have a world block");
    assert_eq!(world_cfg.name, "test_world");
    assert!((world_cfg.gravity() - 9.81).abs() < 1e-6);

    // Should have 2 entity definitions (person, boulder) + 3 spawn commands
    assert_eq!(file.entities.len(), 2, "Expected 2 entity definitions");
    assert_eq!(file.spawn_cmds.len(), 3, "Expected 3 spawn commands");

    let mut world = World::new();
    let result = load_into_world(file, &mut world);

    assert_eq!(result.entity_count, 3, "Expected 3 spawned entities");
    assert!(result.named_entities.contains_key("Elena"));
    assert!(result.named_entities.contains_key("Bob"));
    assert!(result.named_entities.contains_key("Rock1"));

    assert_eq!(world.len(), 3);
}

#[test]
fn test_entity_position_from_scenario() {
    use axiom_lang::evaluator::Position;

    let src = include_str!("../../scenarios/test_basic.ax");
    let file = parse_file(src).unwrap();
    let mut world = World::new();
    let result = load_into_world(file, &mut world);

    let elena_id = result.named_entities["Elena"];
    let pos = world.get::<Position>(elena_id).expect("Elena should have Position");
    assert!((pos.x - 120.0).abs() < 1e-6);
    assert!((pos.y - 200.0).abs() < 1e-6);
}

#[test]
fn test_entity_mass_override_from_scenario() {
    use axiom_lang::evaluator::Mass;

    let src = include_str!("../../scenarios/test_basic.ax");
    let file = parse_file(src).unwrap();
    let mut world = World::new();
    let result = load_into_world(file, &mut world);

    let elena_id = result.named_entities["Elena"];
    let mass = world.get::<Mass>(elena_id).expect("Elena should have Mass");
    // Elena overrides mass to 62 kg
    assert!((mass.0 - 62.0).abs() < 1e-6);

    // Bob uses default person mass of 75 kg
    let bob_id = result.named_entities["Bob"];
    let bob_mass = world.get::<Mass>(bob_id).expect("Bob should have Mass");
    assert!((bob_mass.0 - 75.0).abs() < 1e-6);
}

#[test]
fn test_ecs_world_persistence() {
    let src = include_str!("../../scenarios/test_basic.ax");
    let file = parse_file(src).unwrap();
    let mut world = World::new();
    let result = load_into_world(file, &mut world);

    // Verify all 3 entities still exist after loading
    assert_eq!(world.len(), 3, "World should contain 3 entities");

    // Verify mass is persistent
    let elena_id = result.named_entities["Elena"];
    let rock_id = result.named_entities["Rock1"];

    let elena_mass = world.get::<Mass>(elena_id).expect("Elena mass");
    let rock_mass = world.get::<Mass>(rock_id).expect("Rock mass");

    assert!((elena_mass.0 - 62.0).abs() < 1e-6);
    assert!((rock_mass.0 - 150.0).abs() < 1e-6);
}

#[test]
fn test_entity_temperature_from_scenario() {
    use axiom_lang::evaluator::Temperature;

    let src = include_str!("../../scenarios/test_basic.ax");
    let file = parse_file(src).unwrap();
    let mut world = World::new();
    let result = load_into_world(file, &mut world);

    let elena_id = result.named_entities["Elena"];
    let temp = world.get::<Temperature>(elena_id).expect("Elena should have Temperature");
    // Body temperature: 37°C = 310.15 K
    assert!((temp.0 - 310.15).abs() < 0.1);
}

#[test]
fn test_entity_query_all_positions() {
    use axiom_lang::evaluator::Position;

    let src = include_str!("../../scenarios/test_basic.ax");
    let file = parse_file(src).unwrap();
    let mut world = World::new();
    load_into_world(file, &mut world);

    let mut count = 0;
    for (_id, _pos) in world.query::<&Position>().iter() {
        count += 1;
    }
    assert_eq!(count, 3, "Should be able to query all 3 entities for Position");
}

#[test]
fn test_material_properties_parsing() {
    let src = include_str!("../../scenarios/test_basic.ax");
    let file = parse_file(src).expect("Should parse without errors");

    // Verify materials were parsed
    assert!(!file.materials.is_empty(), "Should have materials defined");
    
    let granite_found = file.materials.iter().any(|m| m.name == "granite");
    assert!(granite_found, "granite material should be defined");
}

#[test]
fn test_scenario_world_config() {
    let src = include_str!("../../scenarios/test_basic.ax");
    let file = parse_file(src).expect("Should parse test_basic.ax");

    let world_cfg = file.world.as_ref().expect("Should have world block");
    assert_eq!(world_cfg.name, "test_world");
    assert!((world_cfg.gravity() - 9.81).abs() < 1e-6);

    // Verify other world properties
    let props = &world_cfg.properties;
    assert!(props.contains_key("tick_rate"), "World should have tick_rate");
    assert!(props.contains_key("ambient_temperature"), "World should have ambient_temperature");
}
    let bob_id = result.named_entities["Bob"];
    let bob_mass = world.get::<Mass>(bob_id).expect("Bob should have Mass");
    assert!((bob_mass.0 - 75.0).abs() < 1e-6);
}
