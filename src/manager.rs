//! Android SDK manager.

use std::collections::HashMap;
use crate::device::{AndroidDevice, DeviceState, DeviceType};
use crate::adb::{AdbConnection, AdbCommand, AdbResult};
use crate::emulator::{Emulator, EmulatorState, EmulatorType};
use crate::package::{AndroidPackage, PackageManager};
use crate::build::{BuildConfig, BuildResult, BuildTool, BuildTarget};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Auto-connect devices
    pub auto_connect: bool,
    /// Auto-start emulators
    pub auto_start_emulators: bool,
    /// Max emulators
    pub max_emulators: usize,
    /// ADB host
    pub adb_host: String,
    /// ADB port
    pub adb_port: u16,
    /// SDK path
    pub sdk_path: String,
    /// Enable build tools
    pub enable_build_tools: bool,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            auto_connect: true,
            auto_start_emulators: false,
            max_emulators: 4,
            adb_host: "localhost".into(),
            adb_port: 5037,
            sdk_path: "/opt/android-sdk".into(),
            enable_build_tools: true,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total devices
    pub total_devices: usize,
    /// Online devices
    pub online_devices: usize,
    /// Offline devices
    pub offline_devices: usize,
    /// Total emulators
    pub total_emulators: usize,
    /// Running emulators
    pub running_emulators: usize,
    /// Total packages
    pub total_packages: usize,
    /// Installed packages
    pub installed_packages: usize,
    /// Total builds
    pub total_builds: usize,
    /// Successful builds
    pub successful_builds: usize,
    /// Failed builds
    pub failed_builds: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Android manager
#[derive(Debug, Clone)]
pub struct AndroidManager {
    /// Devices
    devices: HashMap<String, AndroidDevice>,
    /// ADB connections
    adb_connections: HashMap<String, AdbConnection>,
    /// Emulators
    emulators: HashMap<String, Emulator>,
    /// Package managers per device
    package_managers: HashMap<String, PackageManager>,
    /// Build results
    build_results: Vec<BuildResult>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for AndroidManager {
    fn default() -> Self {
        Self {
            devices: HashMap::new(),
            adb_connections: HashMap::new(),
            emulators: HashMap::new(),
            package_managers: HashMap::new(),
            build_results: vec![],
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_devices: 0,
                online_devices: 0,
                offline_devices: 0,
                total_emulators: 0,
                running_emulators: 0,
                total_packages: 0,
                installed_packages: 0,
                total_builds: 0,
                successful_builds: 0,
                failed_builds: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl AndroidManager {
    /// Create new Android manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            devices: HashMap::new(),
            adb_connections: HashMap::new(),
            emulators: HashMap::new(),
            package_managers: HashMap::new(),
            build_results: vec![],
            options,
            statistics: ManagerStatistics {
                total_devices: 0,
                online_devices: 0,
                offline_devices: 0,
                total_emulators: 0,
                running_emulators: 0,
                total_packages: 0,
                installed_packages: 0,
                total_builds: 0,
                successful_builds: 0,
                failed_builds: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        self.update_statistics();
    }
    
    // Device methods
    
    /// Add device
    pub fn add_device(&mut self, serial: String, device_type: DeviceType) -> String {
        let device = AndroidDevice::new(serial.clone(), device_type);
        self.devices.insert(serial.clone(), device);
        
        // Create ADB connection
        let conn = AdbConnection::new(serial.clone());
        self.adb_connections.insert(serial.clone(), conn);
        
        // Create package manager for device
        self.package_managers.insert(serial.clone(), PackageManager::new());
        
        if self.options.auto_connect {
            self.connect_device(&serial);
        }
        
        self.update_statistics();
        serial
    }
    
    /// Get device
    pub fn get_device(&self, serial: &str) -> Option<&AndroidDevice> {
        self.devices.get(serial)
    }
    
    /// Get device mutably
    pub fn get_device_mut(&mut self, serial: &str) -> Option<&mut AndroidDevice> {
        self.devices.get_mut(serial)
    }
    
    /// Get all devices
    pub fn list_devices(&self) -> Vec<&AndroidDevice> {
        self.devices.values().collect()
    }
    
    /// Remove device
    pub fn remove_device(&mut self, serial: &str) -> bool {
        let removed = self.devices.remove(serial).is_some();
        self.adb_connections.remove(serial);
        self.package_managers.remove(serial);
        if removed {
            self.update_statistics();
        }
        removed
    }
    
    /// Connect device
    pub fn connect_device(&mut self, serial: &str) -> bool {
        if let Some(device) = self.devices.get_mut(serial) {
            device.connect();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Disconnect device
    pub fn disconnect_device(&mut self, serial: &str) -> bool {
        if let Some(device) = self.devices.get_mut(serial) {
            device.disconnect();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get online devices
    pub fn get_online_devices(&self) -> Vec<&AndroidDevice> {
        self.devices.values()
            .filter(|d| d.is_online())
            .collect()
    }
    
    /// Get offline devices
    pub fn get_offline_devices(&self) -> Vec<&AndroidDevice> {
        self.devices.values()
            .filter(|d| d.is_offline())
            .collect()
    }
    
    /// Get devices by type
    pub fn get_devices_by_type(&self, device_type: &DeviceType) -> Vec<&AndroidDevice> {
        self.devices.values()
            .filter(|d| d.device_type == *device_type)
            .collect()
    }
    
    // ADB methods
    
    /// Get ADB connection
    pub fn get_adb_connection(&self, serial: &str) -> Option<&AdbConnection> {
        self.adb_connections.get(serial)
    }
    
    /// Get ADB connection mutably
    pub fn get_adb_connection_mut(&mut self, serial: &str) -> Option<&mut AdbConnection> {
        self.adb_connections.get_mut(serial)
    }
    
    /// Execute ADB command
    pub fn execute_adb_command(&mut self, serial: &str, command: &AdbCommand) -> Result<AdbResult, crate::adb::AdbError> {
        if let Some(conn) = self.adb_connections.get_mut(serial) {
            conn.execute(command)
        } else {
            Err(crate::adb::AdbError::DeviceNotFound)
        }
    }
    
    /// Shell command
    pub fn shell_command(&mut self, serial: &str, command: String) -> Result<AdbResult, crate::adb::AdbError> {
        let adb_cmd = AdbCommand::new("shell".into(), vec![command]);
        self.execute_adb_command(serial, &adb_cmd)
    }
    
    /// Push file
    pub fn push_file(&mut self, serial: &str, local: String, remote: String) -> Result<AdbResult, crate::adb::AdbError> {
        let adb_cmd = AdbCommand::new("push".into(), vec![local, remote]);
        self.execute_adb_command(serial, &adb_cmd)
    }
    
    /// Pull file
    pub fn pull_file(&mut self, serial: &str, remote: String, local: String) -> Result<AdbResult, crate::adb::AdbError> {
        let adb_cmd = AdbCommand::new("pull".into(), vec![remote, local]);
        self.execute_adb_command(serial, &adb_cmd)
    }
    
    /// Install APK
    pub fn install_apk(&mut self, serial: &str, apk_path: String) -> Result<AdbResult, crate::adb::AdbError> {
        let adb_cmd = AdbCommand::new("install".into(), vec![apk_path]);
        self.execute_adb_command(serial, &adb_cmd)
    }
    
    /// Uninstall package
    pub fn uninstall_package_adb(&mut self, serial: &str, package_name: String) -> Result<AdbResult, crate::adb::AdbError> {
        let adb_cmd = AdbCommand::new("uninstall".into(), vec![package_name]);
        self.execute_adb_command(serial, &adb_cmd)
    }
    
    // Emulator methods
    
    /// Create emulator
    pub fn create_emulator(&mut self, avd_name: String) -> String {
        let id = format!("emu-{}", self.emulators.len() + 1);
        let mut emulator = Emulator::new(id.clone());
        emulator.set_name(avd_name.clone());
        emulator.config.avd_name = avd_name;
        
        self.emulators.insert(id.clone(), emulator);
        self.update_statistics();
        id
    }
    
    /// Get emulator
    pub fn get_emulator(&self, emulator_id: &str) -> Option<&Emulator> {
        self.emulators.get(emulator_id)
    }
    
    /// Get emulator mutably
    pub fn get_emulator_mut(&mut self, emulator_id: &str) -> Option<&mut Emulator> {
        self.emulators.get_mut(emulator_id)
    }
    
    /// Get all emulators
    pub fn list_emulators(&self) -> Vec<&Emulator> {
        self.emulators.values().collect()
    }
    
    /// Remove emulator
    pub fn remove_emulator(&mut self, emulator_id: &str) -> bool {
        if self.emulators.remove(emulator_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Start emulator
    pub fn start_emulator(&mut self, emulator_id: &str) -> bool {
        if let Some(emulator) = self.emulators.get_mut(emulator_id) {
            if emulator.start() {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Stop emulator
    pub fn stop_emulator(&mut self, emulator_id: &str) -> bool {
        if let Some(emulator) = self.emulators.get_mut(emulator_id) {
            if emulator.stop() {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Get running emulators
    pub fn get_running_emulators(&self) -> Vec<&Emulator> {
        self.emulators.values()
            .filter(|e| e.is_running())
            .collect()
    }
    
    /// Get stopped emulators
    pub fn get_stopped_emulators(&self) -> Vec<&Emulator> {
        self.emulators.values()
            .filter(|e| e.is_stopped())
            .collect()
    }
    
    /// Get emulators by type
    pub fn get_emulators_by_type(&self, emulator_type: &EmulatorType) -> Vec<&Emulator> {
        self.emulators.values()
            .filter(|e| e.emulator_type == *emulator_type)
            .collect()
    }
    
    // Package methods
    
    /// Install package on device
    pub fn install_package(&mut self, serial: &str, package_name: String, _apk_path: String) -> bool {
        if let Some(pm) = self.package_managers.get_mut(serial) {
            let pkg_name = package_name.clone();
            let package = AndroidPackage::new(pkg_name.clone(), "1.0".into());
            pm.add_package(package);
            pm.install_package(&pkg_name);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Uninstall package from device
    pub fn uninstall_device_package(&mut self, serial: &str, package_name: &str) -> bool {
        if let Some(pm) = self.package_managers.get_mut(serial) {
            if pm.uninstall_package(package_name) {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Get device packages
    pub fn get_device_packages(&self, serial: &str) -> Vec<&AndroidPackage> {
        self.package_managers.get(serial)
            .map(|pm| pm.get_installed_packages())
            .unwrap_or_default()
    }
    
    /// Get device package manager
    pub fn get_device_package_manager(&self, serial: &str) -> Option<&PackageManager> {
        self.package_managers.get(serial)
    }
    
    // Build methods
    
    /// Run build
    pub fn run_build(&mut self, config: BuildConfig) -> BuildResult {
        let _start = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Simulate build
        let duration = 5; // Simulated 5 second build
        let result = BuildResult::success("build/outputs/app.apk".into(), duration);
        
        self.build_results.push(result.clone());
        self.update_statistics();
        result
    }
    
    /// Get build results
    pub fn get_build_results(&self) -> &[BuildResult] {
        &self.build_results
    }
    
    /// Get successful builds
    pub fn get_successful_builds(&self) -> Vec<&BuildResult> {
        self.build_results.iter()
            .filter(|r| r.is_success())
            .collect()
    }
    
    /// Get failed builds
    pub fn get_failed_builds(&self) -> Vec<&BuildResult> {
        self.build_results.iter()
            .filter(|r| r.is_failure())
            .collect()
    }
    
    /// Clear build results
    pub fn clear_build_results(&mut self) {
        self.build_results.clear();
        self.update_statistics();
    }
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_devices = self.devices.len();
        self.statistics.online_devices = self.devices.values()
            .filter(|d| d.is_online())
            .count();
        self.statistics.offline_devices = self.devices.values()
            .filter(|d| d.is_offline())
            .count();
        self.statistics.total_emulators = self.emulators.len();
        self.statistics.running_emulators = self.emulators.values()
            .filter(|e| e.is_running())
            .count();
        
        self.statistics.total_packages = self.package_managers.values()
            .map(|pm| pm.get_package_count())
            .sum();
        self.statistics.installed_packages = self.package_managers.values()
            .map(|pm| pm.get_installed_package_count())
            .sum();
        
        self.statistics.total_builds = self.build_results.len();
        self.statistics.successful_builds = self.build_results.iter()
            .filter(|r| r.is_success())
            .count();
        self.statistics.failed_builds = self.build_results.iter()
            .filter(|r| r.is_failure())
            .count();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            devices: self.devices.values().cloned().collect(),
            emulators: self.emulators.values().cloned().collect(),
            packages: self.package_managers.values()
                .flat_map(|pm| pm.packages.clone())
                .collect(),
            build_results: self.build_results.clone(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.devices.clear();
        for device in export.devices {
            self.devices.insert(device.serial.clone(), device);
        }
        
        self.emulators.clear();
        for emulator in export.emulators {
            self.emulators.insert(emulator.id.clone(), emulator);
        }
        
        // Rebuild package managers
        self.package_managers.clear();
        for device in self.devices.keys() {
            self.package_managers.insert(device.clone(), PackageManager::new());
        }
        for package in export.packages {
            // Find device that has this package (simplified - add to first device)
            if let Some(pm) = self.package_managers.values_mut().next() {
                pm.add_package(package);
            }
        }
        
        self.build_results = export.build_results;
        self.update_statistics();
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Devices
    pub devices: Vec<AndroidDevice>,
    /// Emulators
    pub emulators: Vec<Emulator>,
    /// Packages
    pub packages: Vec<AndroidPackage>,
    /// Build results
    pub build_results: Vec<BuildResult>,
}
