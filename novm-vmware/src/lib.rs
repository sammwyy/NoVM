use novm_core::{detect_os, OSType, VMPlatform};
use unix::VmwareUnix;
use windows::VmwareWin32;

mod unix;
mod windows;

pub fn get_vmware() -> Box<dyn VMPlatform> {
    let os = detect_os();

    match os {
        OSType::Unix => Box::new(VmwareUnix {}),
        OSType::Windows => Box::new(VmwareWin32 {}),
    }
}

pub fn is_vmware() -> bool {
    let os = detect_os();

    match os {
        OSType::Unix => unix::detect(),
        OSType::Windows => windows::detect(),
    }
}
