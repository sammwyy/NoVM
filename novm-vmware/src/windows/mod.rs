use std::path::Path;

use novm_core::{
    are_processes_running, kill_processes_by_name, obfuscate_file, obfuscate_registry_entries,
    remove_registry_key, remove_registry_value, CloakSettings, VMPlatform,
};

use tracing::info;

const VMWARE_PROCESSES: [&str; 7] = [
    "vmtoolsd",
    "vm3dservice",
    "VGAuthService",
    "VMwareService",
    "Vmwaretray",
    "Vmwareuser",
    "TPAutoConnSvc",
];

const VMWARE_REGISTRIES: [(&str, &str); 11] = [
    (r"HARDWARE\DESCRIPTION\System", "SystemBiosVersion"),
    (r"HARDWARE\DESCRIPTION\System\BIOS", "SystemManufacturer"),
    (r"HARDWARE\DESCRIPTION\System\BIOS", "SystemProductName"),
    (
        r"HARDWARE\DEVICEMAP\Scsi\Scsi Port 0\Scsi Bus 0\Target Id 0\Logical Unit Id 0",
        "Identifier",
    ),
    (
        r"HARDWARE\DEVICEMAP\Scsi\Scsi Port 1\Scsi Bus 0\Target Id 0\Logical Unit Id 0",
        "Identifier",
    ),
    (
        r"HARDWARE\DEVICEMAP\Scsi\Scsi Port 2\Scsi Bus 0\Target Id 0\Logical Unit Id 0",
        "Identifier",
    ),
    (
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\WinSAT",
        "PrimaryAdapterString",
    ),
    (
        r"SYSTEM\ControlSet001\Control\SystemInformation",
        "SystemManufacturer",
    ),
    (
        r"SYSTEM\CurrentControlSet\Control\SystemInformation",
        "SystemManufacturer",
    ),
    (
        r"SYSTEM\CurrentControlSet\Control\SystemInformation",
        "SystemProductName",
    ),
    (r"SYSTEM\CurrentControlSet\Services\disk\Enum", "0"),
];

const VMWARE_DIRECTORIES: [&str; 2] = [
    r"C:\Program Files\Common Files\VMware",
    r"C:\Program Files\VMware",
];

const VMWARE_DRIVERS: [&str; 9] = [
    "vmhgfs.sys",
    "vmmemctl.sys",
    "vmmouse.sys",
    "vmrawdsk.sys",
    "vmusbmouse.sys",
    "vm3dmp.sys",
    "vm3dmp_loader.sys",
    "vm3dmp-debug.sys",
    "vm3dmp-stats.sys",
];

const VMWARE_DYN_LIBS: [&str; 18] = [
    "vmhgfs.dll",
    "VMWSU.DLL",
    "vm3dc003.dll",
    "vm3ddevapi64.dll",
    "vm3ddevapi64-debug.dll",
    "vm3ddevapi64-release.dll",
    "vm3ddeapi64-stats.dll",
    "vm3dgl64.dll",
    "vm3dglhelper64.dll",
    "vm3dum64.dll",
    "vm3dum64_10.dll",
    "vm3dum64_10-debug.dll",
    "vm3dum64_10-stats.dll",
    "vm3dum64_loader.dll",
    "vm3dum64-debug.dll",
    "vm3dum64-stats.dll",
    "vmGuestLib.dll",
    "vmGuestLibJava.dll",
];

pub struct VmwareWin32 {}

impl VMPlatform for VmwareWin32 {
    /// Performs cloaking operations for VMware based on settings.
    fn cloak(&self, settings: CloakSettings) {
        if settings.reg {
            info!("Hiding registry keys for VMware...");

            // Obfuscate hardware registries.
            obfuscate_registry_entries(&VMWARE_REGISTRIES);

            // Clean vmware specific registries.
            remove_registry_value(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
                "VMware User Process",
            );
            remove_registry_key(r"SOFTWARE\VMware, Inc.\VMware Tools");
        }

        if settings.files {
            info!("Hiding files for VMware...");

            for directory in VMWARE_DIRECTORIES {
                obfuscate_file(Path::new(directory));
            }
        }

        if settings.drivers {
            info!("Hiding drivers for VMware...");

            // Hide VMware drivers.
            let driver_dir = Path::new(r"C:\Windows\System32\drivers\");
            for driver in VMWARE_DRIVERS {
                obfuscate_file(driver_dir.join(driver));
            }

            // Hide VMware dynamic libraries.
            let dll32 = Path::new(r"C:\Windows\System32");
            for dll in VMWARE_DYN_LIBS {
                obfuscate_file(dll32.join(dll));
            }

            let dll64 = Path::new(r"C:\Windows\SysWOW64");
            for dll in VMWARE_DYN_LIBS {
                obfuscate_file(dll64.join(dll));
            }
        }

        if settings.kill {
            info!("Killing VMware processes");

            // Kill VMware processes.
            kill_processes_by_name(VMWARE_PROCESSES.to_vec());
        }
    }
}

/// Detect if the current environment is VMware.
pub fn detect() -> bool {
    return are_processes_running(VMWARE_PROCESSES.to_vec());
}
