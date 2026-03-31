use crate::ecs::world::World;

/// The core trait every simulation system must implement.
///
/// Systems are the logic layer of the ECS — they read and write component data
/// on every tick. Each system receives a mutable reference to the world and
/// the fixed timestep `dt` (in seconds).
///
/// # Example
/// ```rust
/// use axiom_core::ecs::{World, System};
///
/// struct PrintSystem;
///
/// impl System for PrintSystem {
///     fn name(&self) -> &'static str { "PrintSystem" }
///     fn run(&mut self, _world: &mut World, _dt: f64) {
///         println!("tick");
///     }
/// }
/// ```
pub trait System: Send + Sync {
    /// Human-readable name used in logging and scheduling output.
    fn name(&self) -> &'static str;

    /// Called once per tick with the world reference and the fixed delta-time in seconds.
    fn run(&mut self, world: &mut World, dt: f64);
}
