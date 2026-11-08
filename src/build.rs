//! Android build tools.

/// Build target
#[derive(Debug, Clone, PartialEq)]
pub enum BuildTarget {
    /// APK target
    Apk,
    /// AAB target
    Aab,
    /// Library target
    Library,
    /// Test target
    Test,
    /// Custom target
    Custom(String),
}

/// Build tool
#[derive(Debug, Clone, PartialEq)]
pub enum BuildTool {
    /// Gradle
    Gradle,
    /// Maven
    Maven,
    /// Ant
    Ant,
    /// Custom tool
    Custom(String),
}

/// Build result
#[derive(Debug, Clone)]
pub struct BuildResult {
    /// Build success
    pub success: bool,
    /// Build output path
    pub output_path: Option<String>,
    /// Build error message
    pub error: Option<String>,
    /// Build duration in seconds
    pub duration: u64,
    /// Build timestamp
    pub timestamp: u64,
    /// Build warnings
    pub warnings: Vec<String>,
    /// Build stats
    pub stats: BuildStats,
}

/// Build stats
#[derive(Debug, Clone, Default)]
pub struct BuildStats {
    /// Total files compiled
    pub files_compiled: u32,
    /// Total warnings
    pub total_warnings: u32,
    /// Total errors
    pub total_errors: u32,
    /// Build size in bytes
    pub build_size: u64,
}

impl BuildResult {
    /// Create successful build result
    pub fn success(output_path: String, duration: u64) -> Self {
        Self {
            success: true,
            output_path: Some(output_path),
            error: None,
            duration,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            warnings: vec![],
            stats: BuildStats::default(),
        }
    }
    
    /// Create failed build result
    pub fn failure(error: String, duration: u64) -> Self {
        Self {
            success: false,
            output_path: None,
            error: Some(error),
            duration,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            warnings: vec![],
            stats: BuildStats::default(),
        }
    }
    
    /// Add warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
        self.stats.total_warnings += 1;
    }
    
    /// Check if build was successful
    pub fn is_success(&self) -> bool {
        self.success
    }
    
    /// Check if build failed
    pub fn is_failure(&self) -> bool {
        !self.success
    }
    
    /// Get build age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.timestamp
    }
}

/// Build config
#[derive(Debug, Clone)]
pub struct BuildConfig {
    /// Build tool
    pub tool: BuildTool,
    /// Build target
    pub target: BuildTarget,
    /// Build variant (debug/release)
    pub variant: String,
    /// Build architecture
    pub architecture: String,
    /// Build min SDK
    pub min_sdk: u32,
    /// Build target SDK
    pub target_sdk: u32,
    /// Build output directory
    pub output_dir: String,
    /// Build clean
    pub clean: bool,
    /// Build incremental
    pub incremental: bool,
    /// Build parallel jobs
    pub parallel_jobs: u32,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            tool: BuildTool::Gradle,
            target: BuildTarget::Apk,
            variant: "debug".into(),
            architecture: "arm64-v8a".into(),
            min_sdk: 21,
            target_sdk: 33,
            output_dir: "build/outputs".into(),
            clean: false,
            incremental: true,
            parallel_jobs: 4,
        }
    }
}

impl BuildConfig {
    /// Create new build config
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set tool
    pub fn set_tool(&mut self, tool: BuildTool) {
        self.tool = tool;
    }
    
    /// Set target
    pub fn set_target(&mut self, target: BuildTarget) {
        self.target = target;
    }
    
    /// Set variant
    pub fn set_variant(&mut self, variant: String) {
        self.variant = variant;
    }
    
    /// Set architecture
    pub fn set_architecture(&mut self, arch: String) {
        self.architecture = arch;
    }
    
    /// Set SDK versions
    pub fn set_sdk(&mut self, min_sdk: u32, target_sdk: u32) {
        self.min_sdk = min_sdk;
        self.target_sdk = target_sdk;
    }
    
    /// Enable clean build
    pub fn enable_clean(&mut self) {
        self.clean = true;
    }
    
    /// Disable clean build
    pub fn disable_clean(&mut self) {
        self.clean = false;
    }
    
    /// Enable incremental build
    pub fn enable_incremental(&mut self) {
        self.incremental = true;
    }
    
    /// Disable incremental build
    pub fn disable_incremental(&mut self) {
        self.incremental = false;
    }
    
    /// Set parallel jobs
    pub fn set_parallel_jobs(&mut self, jobs: u32) {
        self.parallel_jobs = jobs;
    }
}
