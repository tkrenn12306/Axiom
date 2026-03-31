/// Runtime configuration for the axiom CLI.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct CliConfig {
    /// Path to the stdlib directory (defaults to ./stdlib)
    pub stdlib_path: Option<String>,
    /// Tick rate in Hz (default: 60)
    pub tick_rate: Option<u64>,
}

impl CliConfig {
    /// Loads the configuration (placeholder for future file/env loading)
    pub fn load() -> Self {
        Self::default()
    }
}
