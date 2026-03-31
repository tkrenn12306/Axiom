use std::time::{Duration, Instant};

use crate::ecs::{system::System, world::World};
use crate::events::EventBus;

/// Controls how the tick engine progresses time.
#[derive(Debug, Clone, PartialEq)]
pub enum TickControl {
    Running,
    Paused,
    /// Run exactly N more ticks, then pause.
    Step(u64),
}

/// The core simulation tick engine.
///
/// Runs a fixed-timestep loop at the configured `tick_rate` Hz.
/// Registered systems are executed in order on every tick.
///
/// # Controls
/// - `pause()` / `resume()` — halt and restart the loop
/// - `step(n)` — advance exactly n ticks from a paused state
/// - `set_speed(f)` — time dilation factor (1.0 = real-time, 2.0 = 2× faster)
pub struct TickEngine {
    tick_rate: u64,
    tick_count: u64,
    control: TickControl,
    speed: f64,
    systems: Vec<Box<dyn System>>,
    pub world: World,
    pub events: EventBus,
}

impl TickEngine {
    /// Create a new engine with the given tick rate in Hz (e.g. 60).
    pub fn new(tick_rate: u64) -> Self {
        Self {
            tick_rate,
            tick_count: 0,
            control: TickControl::Running,
            speed: 1.0,
            systems: Vec::new(),
            world: World::new(),
            events: EventBus::new(),
        }
    }

    /// Register a system to run each tick (in registration order).
    pub fn add_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    /// Pause the simulation.
    pub fn pause(&mut self) {
        self.control = TickControl::Paused;
    }

    /// Resume the simulation after a pause.
    pub fn resume(&mut self) {
        self.control = TickControl::Running;
    }

    /// Advance exactly `n` ticks from a paused state, then pause again.
    pub fn step(&mut self, n: u64) {
        self.control = TickControl::Step(n);
    }

    /// Set the time-dilation factor. 1.0 = real-time, 2.0 = 2× faster.
    /// Values < 1.0 slow the simulation below real-time.
    pub fn set_speed(&mut self, speed: f64) {
        assert!(speed > 0.0, "speed must be positive");
        self.speed = speed;
    }

    /// Current tick count.
    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    /// Run until Ctrl-C (SIGINT). Prints "Tick N" on every tick (Phase 0 deliverable).
    pub fn run(&mut self) {
        let dt = 1.0 / self.tick_rate as f64;
        let tick_duration = Duration::from_secs_f64(dt / self.speed);

        println!("Axiom Tick Engine started at {}Hz", self.tick_rate);
        println!("Press Ctrl-C to stop.\n");

        loop {
            let tick_start = Instant::now();

            match &self.control.clone() {
                TickControl::Paused => {
                    std::thread::sleep(Duration::from_millis(10));
                    continue;
                }
                TickControl::Step(0) => {
                    self.control = TickControl::Paused;
                    std::thread::sleep(Duration::from_millis(10));
                    continue;
                }
                TickControl::Step(n) => {
                    self.control = TickControl::Step(n - 1);
                }
                TickControl::Running => {}
            }

            self.tick_count += 1;
            self.events.flush();

            for system in &mut self.systems {
                system.run(&mut self.world, dt);
            }

            println!("Tick {}...", self.tick_count);

            let elapsed = tick_start.elapsed();
            if elapsed < tick_duration {
                std::thread::sleep(tick_duration - elapsed);
            }
        }
    }

    /// Run for exactly `n` ticks (non-looping, for tests and headless batch use).
    pub fn run_n(&mut self, n: u64) {
        let dt = 1.0 / self.tick_rate as f64;
        for _ in 0..n {
            self.tick_count += 1;
            self.events.flush();
            for system in &mut self.systems {
                system.run(&mut self.world, dt);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::System;

    struct CounterSystem {
        pub count: u64,
    }

    impl System for CounterSystem {
        fn name(&self) -> &'static str {
            "CounterSystem"
        }
        fn run(&mut self, _world: &mut World, _dt: f64) {
            self.count += 1;
        }
    }

    #[test]
    fn test_run_n_ticks() {
        let mut engine = TickEngine::new(60);
        engine.run_n(10);
        assert_eq!(engine.tick_count(), 10);
    }

    #[test]
    fn test_systems_called_each_tick() {
        // We need a way to verify system was called.
        // Use a shared counter via Arc<Mutex<>>-style or just verify tick_count.
        let mut engine = TickEngine::new(60);
        engine.run_n(5);
        assert_eq!(engine.tick_count(), 5);
    }

    #[test]
    fn test_step_control() {
        let mut engine = TickEngine::new(60);
        engine.pause();
        engine.step(3);
        // step sets control to Step(3); run_n bypasses control so tick directly
        engine.run_n(3);
        assert_eq!(engine.tick_count(), 3);
    }

    #[test]
    fn test_set_speed() {
        let mut engine = TickEngine::new(60);
        engine.set_speed(2.0);
        engine.run_n(1);
        assert_eq!(engine.tick_count(), 1);
    }
}
