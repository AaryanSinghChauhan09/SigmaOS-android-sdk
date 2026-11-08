//! Command-line options for Android SDK.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Default mode
    Default,
    /// Devices mode
    Devices,
    /// Emulators mode
    Emulators,
    /// Packages mode
    Packages,
    /// Build mode
    Build,
    /// Connect mode
    Connect(String),
    /// Install mode
    Install(String, String),
    /// Status mode
    Status,
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Config file path
    pub config_path: String,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            config_path: String::new(),
            verbose: false,
            quiet: false,
        }
    }
}
