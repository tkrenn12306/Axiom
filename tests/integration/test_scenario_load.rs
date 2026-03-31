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
