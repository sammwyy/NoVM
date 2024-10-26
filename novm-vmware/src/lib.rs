#[cfg(target_os = "linux")]
mod unix;
#[cfg(target_os = "windows")]
mod windows;

use novm_core::VMPlatform;
#[cfg(target_os = "linux")]
use unix::VmwareUnix;
#[cfg(target_os = "windows")]
use windows::VmwareWin32;

pub fn get_vmware() -> Box<dyn VMPlatform> {
    #[cfg(target_os = "windows")]
    return Box::new(VmwareWin32 {});

    #[cfg(target_os = "linux")]
    return Box::new(VmwareUnix {});
}

pub fn is_vmware() -> bool {
    #[cfg(target_os = "windows")]
    return windows::detect();

    #[cfg(target_os = "linux")]
    return unix::detect();
}
