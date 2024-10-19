use novm_core::{CloakSettings, VMPlatform};

pub struct VmwareUnix {}

impl VMPlatform for VmwareUnix {
    /// Performs cloaking operations for VMware based on settings.
    fn cloak(&self, _settings: CloakSettings) {
        todo!("Not implemented yet.");
    }
}

/// Detect if the current environment is VMware.
pub fn detect() -> bool {
    todo!("Not implemented yet.");
}
