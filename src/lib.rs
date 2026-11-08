//! SigmaOS-android-sdk — Android SDK/ADB component.
//!
//! Models Android SDK management, ADB device connections, emulator
//! management, package management, and build tools for SigmaOS.

pub mod device;
pub mod adb;
pub mod emulator;
pub mod package;
pub mod build;
pub mod manager;
pub mod options;

pub use device::{AndroidDevice, DeviceState, DeviceType, DeviceInfo};
pub use adb::{AdbConnection, AdbCommand, AdbResult, AdbError};
pub use emulator::{Emulator, EmulatorConfig, EmulatorState, EmulatorType};
pub use package::{AndroidPackage, PackageInfo, PackageManager};
pub use build::{BuildConfig, BuildTarget, BuildResult, BuildTool};
pub use manager::{AndroidManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_device_state() {
        let states = vec![
            DeviceState::Offline,
            DeviceState::Unauthorized,
            DeviceState::Authorizing,
            DeviceState::Online,
            DeviceState::Bootloader,
            DeviceState::Recovery,
            DeviceState::Sideload,
        ];
        assert_eq!(states.len(), 7);
    }
    
    #[test]
    fn test_device_type() {
        let types = vec![
            DeviceType::Phone,
            DeviceType::Tablet,
            DeviceType::TV,
            DeviceType::Wear,
            DeviceType::Emulator,
            DeviceType::Auto,
        ];
        assert_eq!(types.len(), 6);
    }
    
    #[test]
    fn test_adb_command() {
        let cmd = AdbCommand::new("shell".into(), vec!["ls".into(), "-la".into()]);
        assert_eq!(cmd.command, "shell");
        assert_eq!(cmd.args.len(), 2);
    }
    
    #[test]
    fn test_emulator_state() {
        let states = vec![
            EmulatorState::Stopped,
            EmulatorState::Starting,
            EmulatorState::Running,
            EmulatorState::Paused,
            EmulatorState::Saving,
            EmulatorState::Error,
        ];
        assert_eq!(states.len(), 6);
    }
    
    #[test]
    fn test_device_creation() {
        let device = AndroidDevice::new("emulator-5554".into(), DeviceType::Emulator);
        assert_eq!(device.serial, "emulator-5554");
        assert_eq!(device.device_type, DeviceType::Emulator);
        assert_eq!(device.state, DeviceState::Offline);
    }
    
    #[test]
    fn test_adb_connection_creation() {
        let conn = AdbConnection::new("emulator-5554".into());
        assert_eq!(conn.serial, "emulator-5554");
        assert!(!conn.connected);
    }
    
    #[test]
    fn test_emulator_creation() {
        let emu = Emulator::new("emu-1".into());
        assert_eq!(emu.id, "emu-1");
        assert_eq!(emu.state, EmulatorState::Stopped);
    }
    
    #[test]
    fn test_package_info() {
        let pkg = AndroidPackage::new("com.example.app".into(), "1.0".into());
        assert_eq!(pkg.package_name, "com.example.app");
        assert_eq!(pkg.version, "1.0");
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = AndroidManager::new();
        assert!(manager.list_devices().is_empty());
        assert!(manager.list_emulators().is_empty());
    }
    
    #[test]
    fn test_connect_device() {
        let mut manager = AndroidManager::new();
        let device_id = manager.add_device("emulator-5554".into(), DeviceType::Emulator);
        
        assert!(manager.connect_device(&device_id));
        assert_eq!(manager.get_device(&device_id).unwrap().state, DeviceState::Online);
    }
    
    #[test]
    fn test_disconnect_device() {
        let mut manager = AndroidManager::new();
        let device_id = manager.add_device("emulator-5554".into(), DeviceType::Emulator);
        manager.connect_device(&device_id);
        
        assert!(manager.disconnect_device(&device_id));
        assert_eq!(manager.get_device(&device_id).unwrap().state, DeviceState::Offline);
    }
    
    #[test]
    fn test_create_emulator() {
        let mut manager = AndroidManager::new();
        let emu_id = manager.create_emulator("Pixel_6".into());
        
        assert!(manager.get_emulator(&emu_id).is_some());
        assert_eq!(manager.list_emulators().len(), 1);
    }
    
    #[test]
    fn test_start_emulator() {
        let mut manager = AndroidManager::new();
        let emu_id = manager.create_emulator("Pixel_6".into());
        
        assert!(manager.start_emulator(&emu_id));
        assert_eq!(manager.get_emulator(&emu_id).unwrap().state, EmulatorState::Starting);
    }
    
    #[test]
    fn test_install_package() {
        let mut manager = AndroidManager::new();
        let device_id = manager.add_device("emulator-5554".into(), DeviceType::Emulator);
        
        assert!(manager.install_package(&device_id, "com.example.app".into(), "/path/app.apk".into()));
        assert_eq!(manager.get_device_packages(&device_id).len(), 1);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = AndroidManager::new();
        manager.add_device("emulator-5554".into(), DeviceType::Emulator);
        manager.create_emulator("Pixel_6".into());
        
        let export = manager.export_data();
        
        let mut manager2 = AndroidManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_devices().len(), 1);
        assert_eq!(manager2.list_emulators().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = AndroidManager::new();
        manager.add_device("emu1".into(), DeviceType::Emulator);
        manager.add_device("phone1".into(), DeviceType::Phone);
        manager.create_emulator("Pixel_6".into());
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_devices, 2);
        assert_eq!(stats.total_emulators, 1);
    }
}
