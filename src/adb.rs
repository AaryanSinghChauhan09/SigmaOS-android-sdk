//! ADB (Android Debug Bridge) for device communication.

/// ADB command
#[derive(Debug, Clone)]
pub struct AdbCommand {
    /// Command name
    pub command: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Command timeout in seconds
    pub timeout: u64,
    /// Command created timestamp
    pub created_at: u64,
}

impl AdbCommand {
    /// Create new ADB command
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self {
            command,
            args,
            timeout: 30,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Set timeout
    pub fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }
    
    /// Get command string
    pub fn to_string(&self) -> String {
        let mut cmd = self.command.clone();
        for arg in &self.args {
            cmd.push(' ');
            cmd.push_str(arg);
        }
        cmd
    }
}

/// ADB result
#[derive(Debug, Clone)]
pub struct AdbResult {
    /// Result success
    pub success: bool,
    /// Result output
    pub output: String,
    /// Result error
    pub error: Option<String>,
    /// Result exit code
    pub exit_code: i32,
    /// Result duration in milliseconds
    pub duration_ms: u64,
    /// Result timestamp
    pub timestamp: u64,
}

impl AdbResult {
    /// Create successful result
    pub fn success(output: String) -> Self {
        Self {
            success: true,
            output,
            error: None,
            exit_code: 0,
            duration_ms: 0,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Create failed result
    pub fn failure(error: String, exit_code: i32) -> Self {
        Self {
            success: false,
            output: String::new(),
            error: Some(error),
            exit_code,
            duration_ms: 0,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// ADB error
#[derive(Debug, Clone, PartialEq)]
pub enum AdbError {
    /// Device not found
    DeviceNotFound,
    /// Device offline
    DeviceOffline,
    /// Command failed
    CommandFailed(String),
    /// Timeout
    Timeout,
    /// Connection refused
    ConnectionRefused,
    /// Custom error
    Custom(String),
}

/// ADB connection
#[derive(Debug, Clone)]
pub struct AdbConnection {
    /// Device serial
    pub serial: String,
    /// Connection state
    pub connected: bool,
    /// Connection host
    pub host: String,
    /// Connection port
    pub port: u16,
    /// Last command timestamp
    pub last_command: Option<u64>,
    /// Command count
    pub command_count: u64,
    /// Error count
    pub error_count: u64,
    /// Connection created timestamp
    pub created_at: u64,
}

impl AdbConnection {
    /// Create new ADB connection
    pub fn new(serial: String) -> Self {
        Self {
            serial,
            connected: false,
            host: "localhost".into(),
            port: 5037,
            last_command: None,
            command_count: 0,
            error_count: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Connect
    pub fn connect(&mut self) {
        self.connected = true;
    }
    
    /// Disconnect
    pub fn disconnect(&mut self) {
        self.connected = false;
    }
    
    /// Execute command
    pub fn execute(&mut self, command: &AdbCommand) -> Result<AdbResult, AdbError> {
        if !self.connected {
            return Err(AdbError::DeviceOffline);
        }
        
        self.command_count += 1;
        self.last_command = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        
        // Simulate command execution
        Ok(AdbResult::success(format!("Executed: {}", command.to_string())))
    }
    
    /// Record error
    pub fn record_error(&mut self) {
        self.error_count += 1;
    }
    
    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }
    
    /// Get connection age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get error rate
    pub fn get_error_rate(&self) -> f64 {
        if self.command_count == 0 {
            0.0
        } else {
            (self.error_count as f64 / self.command_count as f64) * 100.0
        }
    }
}
