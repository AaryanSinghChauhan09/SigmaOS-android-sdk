//! Android package management.

/// Package info
#[derive(Debug, Clone)]
pub struct PackageInfo {
    /// Package name
    pub package_name: String,
    /// Version name
    pub version_name: String,
    /// Version code
    pub version_code: u32,
    /// First install time
    pub first_install_time: u64,
    /// Last update time
    pub last_update_time: u64,
    /// APK path
    pub apk_path: String,
    /// Data directory
    pub data_dir: String,
    /// Cache directory
    pub cache_dir: String,
    /// Permissions
    pub permissions: Vec<String>,
    /// Features
    pub features: Vec<String>,
    /// Shared user ID
    pub shared_user_id: Option<String>,
    /// Install location
    pub install_location: InstallLocation,
}

/// Install location
#[derive(Debug, Clone, PartialEq)]
pub enum InstallLocation {
    /// Internal storage
    Internal,
    /// External storage
    External,
    /// SD card
    SdCard,
    /// Custom location
    Custom(String),
}

/// Android package
#[derive(Debug, Clone)]
pub struct AndroidPackage {
    /// Package name
    pub package_name: String,
    /// Version
    pub version: String,
    /// Package installed state
    pub installed: bool,
    /// Package enabled state
    pub enabled: bool,
    /// Package info
    pub info: Option<PackageInfo>,
    /// Package installed timestamp
    pub installed_at: Option<u64>,
    /// Package updated timestamp
    pub updated_at: Option<u64>,
}

impl AndroidPackage {
    /// Create new Android package
    pub fn new(package_name: String, version: String) -> Self {
        Self {
            package_name,
            version,
            installed: false,
            enabled: true,
            info: None,
            installed_at: None,
            updated_at: None,
        }
    }
    
    /// Set installed
    pub fn set_installed(&mut self) {
        self.installed = true;
        self.installed_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        self.updated_at = self.installed_at;
    }
    
    /// Set uninstalled
    pub fn set_uninstalled(&mut self) {
        self.installed = false;
        self.updated_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
    }
    
    /// Set enabled
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.updated_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
    }
    
    /// Set info
    pub fn set_info(&mut self, info: PackageInfo) {
        self.info = Some(info);
        self.updated_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
    }
    
    /// Check if installed
    pub fn is_installed(&self) -> bool {
        self.installed
    }
    
    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Check if disabled
    pub fn is_disabled(&self) -> bool {
        !self.enabled
    }
    
    /// Get package age in seconds
    pub fn get_age_seconds(&self) -> Option<u64> {
        self.installed_at.map(|installed| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() - installed
        })
    }
    
    /// Get time since update in seconds
    pub fn get_time_since_update(&self) -> Option<u64> {
        self.updated_at.map(|updated| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() - updated
        })
    }
}

/// Package manager
#[derive(Debug, Clone, Default)]
pub struct PackageManager {
    /// Packages
    pub packages: Vec<AndroidPackage>,
}

impl PackageManager {
    /// Create new package manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add package
    pub fn add_package(&mut self, package: AndroidPackage) -> bool {
        if self.packages.iter().any(|p| p.package_name == package.package_name) {
            return false;
        }
        self.packages.push(package);
        true
    }
    
    /// Get package
    pub fn get_package(&self, package_name: &str) -> Option<&AndroidPackage> {
        self.packages.iter().find(|p| p.package_name == package_name)
    }
    
    /// Get package mutably
    pub fn get_package_mut(&mut self, package_name: &str) -> Option<&mut AndroidPackage> {
        self.packages.iter_mut().find(|p| p.package_name == package_name)
    }
    
    /// Remove package
    pub fn remove_package(&mut self, package_name: &str) -> bool {
        if let Some(pos) = self.packages.iter().position(|p| p.package_name == package_name) {
            self.packages.remove(pos);
            return true;
        }
        false
    }
    
    /// Install package
    pub fn install_package(&mut self, package_name: &str) -> bool {
        if let Some(package) = self.get_package_mut(package_name) {
            package.set_installed();
            return true;
        }
        false
    }
    
    /// Uninstall package
    pub fn uninstall_package(&mut self, package_name: &str) -> bool {
        if let Some(package) = self.get_package_mut(package_name) {
            package.set_uninstalled();
            return true;
        }
        false
    }
    
    /// Enable package
    pub fn enable_package(&mut self, package_name: &str) -> bool {
        if let Some(package) = self.get_package_mut(package_name) {
            package.set_enabled(true);
            return true;
        }
        false
    }
    
    /// Disable package
    pub fn disable_package(&mut self, package_name: &str) -> bool {
        if let Some(package) = self.get_package_mut(package_name) {
            package.set_enabled(false);
            return true;
        }
        false
    }
    
    /// Get installed packages
    pub fn get_installed_packages(&self) -> Vec<&AndroidPackage> {
        self.packages.iter().filter(|p| p.is_installed()).collect()
    }
    
    /// Get enabled packages
    pub fn get_enabled_packages(&self) -> Vec<&AndroidPackage> {
        self.packages.iter().filter(|p| p.is_enabled()).collect()
    }
    
    /// Get disabled packages
    pub fn get_disabled_packages(&self) -> Vec<&AndroidPackage> {
        self.packages.iter().filter(|p| p.is_disabled()).collect()
    }
    
    /// Search packages
    pub fn search_packages(&self, query: &str) -> Vec<&AndroidPackage> {
        let query_lower = query.to_lowercase();
        self.packages.iter()
            .filter(|p| p.package_name.to_lowercase().contains(&query_lower) ||
                       p.version.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    /// Get package count
    pub fn get_package_count(&self) -> usize {
        self.packages.len()
    }
    
    /// Get installed package count
    pub fn get_installed_package_count(&self) -> usize {
        self.get_installed_packages().len()
    }
    
    /// Check if package exists
    pub fn has_package(&self, package_name: &str) -> bool {
        self.packages.iter().any(|p| p.package_name == package_name)
    }
    
    /// Clear all packages
    pub fn clear(&mut self) {
        self.packages.clear();
    }
}
