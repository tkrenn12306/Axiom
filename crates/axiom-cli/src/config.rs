/// Runtime configuration for the axiom CLI.
#[derive(Debug, Default)]
pub struct CliConfig {
    /// Path to the stdlib directory (defaults to ./stdlib)
    pub stdlib_path: Option<String>,
    /// Tick rate in Hz (default: 60)
    pub tick_rate: Option<u64>,
}
