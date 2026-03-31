// Physics unit test skeleton for free-fall validation.
// Actual physics implementation is in Phase 2 (axiom-physics).
// These tests will become active when GravitySystem is implemented.

use axiom_core::World;
use axiom_lang::{load_into_world, parse_file, evaluator::{Position, Velocity, Mass, Height}};

#[test]
fn test_free_fall_kinematics_formula() {
    // Validate: h = ½ · g · t²
    let g = 9.81_f64; // m/s²
    let t = 2.0_f64;  // seconds
    let expected_height = 0.5 * g * t * t;
    assert!((expected_height - 19.62).abs() < 1e-6);
}

#[test]
fn test_free_fall_velocity_formula() {
    // Validate: v = g · t
    let g = 9.81_f64;
    let t = 3.0_f64;
    let expected_velocity = g * t;
    assert!((expected_velocity - 29.43).abs() < 1e-6);
}

#[test]
fn test_impact_velocity_from_height() {
    // Validate: v = sqrt(2 · g · h)
    let g = 9.81_f64;
    let h = 20.0_f64; // meters
    let expected_v = (2.0 * g * h).sqrt();
    assert!((expected_v - 19.809).abs() < 1e-3);
}

// ─────────────────────────────────────
// Phase 2 ECS Integration Tests
// ─────────────────────────────────────

#[test]
fn test_entity_has_velocity_component() {
    let src = r#"
        world "test" {
            gravity: 9.81
            tick_rate: 60
        }
        
        entity rock {
            type: rigid_body
            mass: 10
        }
        
        spawn rock "TestRock" at (100, 100) {
            velocity: (0, -5)
        }
    "#;

    let file = parse_file(src).unwrap();
    let mut world = World::new();
    let result = load_into_world(file, &mut world);

    let rock_id = result.named_entities["TestRock"];
    
    // Verify Position exists
    let pos = world.get::<Position>(rock_id).expect("Rock should have Position");
    assert!((pos.x - 100.0).abs() < 1e-6);
    assert!((pos.y - 100.0).abs() < 1e-6);

    // Verify Velocity exists  
    let vel = world.get::<Velocity>(rock_id).expect("Rock should have Velocity");
    assert!((vel.vx - 0.0).abs() < 1e-6);
    assert!((vel.vy - (-5.0)).abs() < 1e-6);

    // Verify Mass exists
    let mass = world.get::<Mass>(rock_id).expect("Rock should have Mass");
    assert!((mass.0 - 10.0).abs() < 1e-6);
}

#[test]
fn test_world_config_gravity() {
    let src = r#"
        world "gravity_test" {
            gravity: 9.81
            tick_rate: 60
        }
    "#;

    let file = parse_file(src).unwrap();
    let world_cfg = file.world.as_ref().expect("Should have world config");
    
    let gravity = world_cfg.gravity();
    assert!((gravity - 9.81).abs() < 1e-6, "Gravity should be 9.81 m/s²");
}

#[test]
fn test_multiple_entities_free_fall() {
    let src = r#"
        world "multi_fall" {
            gravity: 9.81
        }
        
        entity testobj {
            type: rigid_body
            mass: 5
        }
        
        spawn testobj "Obj1" at (50, 50) { velocity: (0, 0) }
        spawn testobj "Obj2" at (100, 100) { velocity: (0, -2) }
        spawn testobj "Obj3" at (150, 150) { velocity: (0, -10) }
    "#;

    let file = parse_file(src).unwrap();
    let mut world = World::new();
    let result = load_into_world(file, &mut world);

    // All 3 entities should spawn with different velocities
    assert_eq!(world.len(), 3);

    let obj2_id = result.named_entities["Obj2"];
    let vel = world.get::<Velocity>(obj2_id).expect("Obj2 should have Velocity");
    assert!((vel.vy - (-2.0)).abs() < 1e-6);

    let obj3_id = result.named_entities["Obj3"];
    let vel3 = world.get::<Velocity>(obj3_id).expect("Obj3 should have Velocity");
    assert!((vel3.vy - (-10.0)).abs() < 1e-6);
}

#[test]
fn test_entity_height_property() {
    let src = r#"
        world "test" {
            gravity: 9.81
        }
        
        entity person {
            type: humanoid
            mass: 70
            height: 1.80
        }
        
        spawn person "Alice" at (200, 200)
    "#;

    let file = parse_file(src).unwrap();
    let mut world = World::new();
    let result = load_into_world(file, &mut world);

    let alice_id = result.named_entities["Alice"];
    let height = world.get::<Height>(alice_id).expect("Alice should have Height");
    assert!((height.0 - 1.80).abs() < 1e-6);
}
