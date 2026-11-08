//! Android device definitions.

/// Device state
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceState {
    /// Offline device
    Offline,
    /// Unauthorized device
    Unauthorized,
    /// Authorizing device
    Authorizing,
    /// Online device
    Online,
    /// Bootloader mode
    Bootloader,
    /// Recovery mode
    Recovery,
    /// Sideload mode
    Sideload,
}

/// Device type
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceType {
    /// Phone device
    Phone,
    /// Tablet device
    Tablet,
    /// TV device
    TV,
    /// Wear device
    Wear,
    /// Emulator device
    Emulator,
    /// Auto device
    Auto,
}

/// Device info
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// Device model
    pub model: String,
    /// Device product
    pub product: String,
    /// Device manufacturer
    pub manufacturer: String,
    /// Android version
    pub android_version: String,
    /// API level
    pub api_level: u32,
    /// Device ABI
    pub abi: String,
    /// Device screen size
    pub screen_size: String,
    /// Device density
    pub density: u32,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            model: String::new(),
            product: String::new(),
            manufacturer: String::new(),
            android_version: String::new(),
            api_level: 0,
            abi: String::new(),
            screen_size: String::new(),
            density: 0,
        }
    }
}

/// Android device
#[derive(Debug, Clone)]
pub struct AndroidDevice {
    /// Device serial
    pub serial: String,
    /// Device type
    pub device_type: DeviceType,
    /// Device state
    pub state: DeviceState,
    /// Device info
    pub info: DeviceInfo,
    /// Device connected timestamp
    pub connected_at: Option<u64>,
    /// Device last seen timestamp
    pub last_seen: u64,
    /// Device battery level
    pub battery_level: Option<u8>,
    /// Device charging state
    pub charging: bool,
}

impl AndroidDevice {
    /// Create new Android device
    pub fn new(serial: String, device_type: DeviceType) -> Self {
        Self {
            serial,
            device_type,
            state: DeviceState::Offline,
            info: DeviceInfo::default(),
            connected_at: None,
            last_seen: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            battery_level: None,
            charging: false,
        }
    }
    
    /// Set device info
    pub fn set_info(&mut self, info: DeviceInfo) {
        self.info = info;
        self.touch();
    }
    
    /// Connect device
    pub fn connect(&mut self) {
        self.state = DeviceState::Online;
        self.connected_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        self.touch();
    }
    
    /// Disconnect device
    pub fn disconnect(&mut self) {
        self.state = DeviceState::Offline;
        self.connected_at = None;
        self.touch();
    }
    
    /// Set state
    pub fn set_state(&mut self, state: DeviceState) {
        self.state = state;
        self.touch();
    }
    
    /// Set battery level
    pub fn set_battery_level(&mut self, level: u8) {
        self.battery_level = Some(level);
        self.touch();
    }
    
    /// Set charging state
    pub fn set_charging(&mut self, charging: bool) {
        self.charging = charging;
        self.touch();
    }
    
    /// Touch device (update last seen)
    pub fn touch(&mut self) {
        self.last_seen = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if device is online
    pub fn is_online(&self) -> bool {
        self.state == DeviceState::Online
    }
    
    /// Check if device is offline
    pub fn is_offline(&self) -> bool {
        self.state == DeviceState::Offline
    }
    
    /// Check if device is connected
    pub fn is_connected(&self) -> bool {
        self.connected_at.is_some()
    }
    
    /// Get device age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.last_seen
    }
    
    /// Get connection duration in seconds
    pub fn get_connection_duration(&self) -> Option<u64> {
        self.connected_at.map(|start| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() - start
        })
    }
}
