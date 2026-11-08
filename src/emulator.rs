//! Android emulator management.

/// Emulator state
#[derive(Debug, Clone, PartialEq)]
pub enum EmulatorState {
    /// Stopped emulator
    Stopped,
    /// Starting emulator
    Starting,
    /// Running emulator
    Running,
    /// Paused emulator
    Paused,
    /// Saving emulator
    Saving,
    /// Error emulator
    Error,
}

/// Emulator type
#[derive(Debug, Clone, PartialEq)]
pub enum EmulatorType {
    /// Phone emulator
    Phone,
    /// Tablet emulator
    Tablet,
    /// TV emulator
    TV,
    /// Wear emulator
    Wear,
    /// Auto emulator
    Auto,
    /// Custom emulator
    Custom(String),
}

/// Emulator config
#[derive(Debug, Clone)]
pub struct EmulatorConfig {
    /// AVD name
    pub avd_name: String,
    /// System image
    pub system_image: String,
    /// RAM in MB
    pub ram_mb: u64,
    /// VM heap in MB
    pub vm_heap_mb: u64,
    /// SD card size in MB
    pub sdcard_mb: u64,
    /// Skin name
    pub skin: String,
    /// GPU mode
    pub gpu_mode: String,
    /// Camera back
    pub camera_back: String,
    /// Camera front
    pub camera_front: String,
    /// Network speed
    pub network_speed: String,
    /// Network delay
    pub network_delay: String,
}

impl Default for EmulatorConfig {
    fn default() -> Self {
        Self {
            avd_name: String::new(),
            system_image: "system-images;android-33;google_apis;x86_64".into(),
            ram_mb: 2048,
            vm_heap_mb: 512,
            sdcard_mb: 512,
            skin: "pixel_6".into(),
            gpu_mode: "auto".into(),
            camera_back: "emulated".into(),
            camera_front: "emulated".into(),
            network_speed: "full".into(),
            network_delay: "none".into(),
        }
    }
}

/// Emulator
#[derive(Debug, Clone)]
pub struct Emulator {
    /// Emulator ID
    pub id: String,
    /// Emulator name
    pub name: String,
    /// Emulator type
    pub emulator_type: EmulatorType,
    /// Emulator state
    pub state: EmulatorState,
    /// Emulator config
    pub config: EmulatorConfig,
    /// Emulator serial
    pub serial: Option<String>,
    /// Emulator PID
    pub pid: Option<u32>,
    /// Emulator created timestamp
    pub created_at: u64,
    /// Emulator started timestamp
    pub started_at: Option<u64>,
    /// Emulator last activity timestamp
    pub last_activity: u64,
    /// Emulator snapshot count
    pub snapshot_count: u64,
}

impl Emulator {
    /// Create new emulator
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: String::new(),
            emulator_type: EmulatorType::Phone,
            state: EmulatorState::Stopped,
            config: EmulatorConfig::default(),
            serial: None,
            pid: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            started_at: None,
            last_activity: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            snapshot_count: 0,
        }
    }
    
    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.touch();
    }
    
    /// Set type
    pub fn set_type(&mut self, emulator_type: EmulatorType) {
        self.emulator_type = emulator_type;
        self.touch();
    }
    
    /// Set config
    pub fn set_config(&mut self, config: EmulatorConfig) {
        self.config = config;
        self.touch();
    }
    
    /// Start emulator
    pub fn start(&mut self) -> bool {
        if self.state == EmulatorState::Stopped || self.state == EmulatorState::Error {
            self.state = EmulatorState::Starting;
            self.started_at = Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());
            self.touch();
            return true;
        }
        false
    }
    
    /// Stop emulator
    pub fn stop(&mut self) -> bool {
        if self.state == EmulatorState::Running || self.state == EmulatorState::Paused {
            self.state = EmulatorState::Stopped;
            self.pid = None;
            self.serial = None;
            self.touch();
            return true;
        }
        false
    }
    
    /// Pause emulator
    pub fn pause(&mut self) -> bool {
        if self.state == EmulatorState::Running {
            self.state = EmulatorState::Paused;
            self.touch();
            return true;
        }
        false
    }
    
    /// Resume emulator
    pub fn resume(&mut self) -> bool {
        if self.state == EmulatorState::Paused {
            self.state = EmulatorState::Running;
            self.touch();
            return true;
        }
        false
    }
    
    /// Set running state
    pub fn set_running(&mut self, serial: String, pid: u32) {
        self.state = EmulatorState::Running;
        self.serial = Some(serial);
        self.pid = Some(pid);
        self.touch();
    }
    
    /// Set error state
    pub fn set_error(&mut self) {
        self.state = EmulatorState::Error;
        self.touch();
    }
    
    /// Increment snapshot count
    pub fn increment_snapshot_count(&mut self) {
        self.snapshot_count += 1;
        self.touch();
    }
    
    /// Touch emulator (update last activity)
    pub fn touch(&mut self) {
        self.last_activity = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if emulator is running
    pub fn is_running(&self) -> bool {
        self.state == EmulatorState::Running
    }
    
    /// Check if emulator is stopped
    pub fn is_stopped(&self) -> bool {
        self.state == EmulatorState::Stopped
    }
    
    /// Check if emulator is paused
    pub fn is_paused(&self) -> bool {
        self.state == EmulatorState::Paused
    }
    
    /// Check if emulator has error
    pub fn has_error(&self) -> bool {
        self.state == EmulatorState::Error
    }
    
    /// Get emulator age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get uptime in seconds
    pub fn get_uptime(&self) -> Option<u64> {
        self.started_at.map(|start| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() - start
        })
    }
    
    /// Get time since last activity in seconds
    pub fn get_time_since_activity(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.last_activity
    }
}
