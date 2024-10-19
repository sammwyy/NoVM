use std::{ffi::OsStr, fs, path::Path, process::Command};

use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sysinfo::System;
use tracing::{error, info, warn};
use winreg::enums::HKEY_LOCAL_MACHINE;
#[cfg(target_os = "windows")]
use winreg::{
    enums::{KEY_QUERY_VALUE, KEY_SET_VALUE},
    RegKey,
};

/// Struct representing settings for the cloaking operation.
pub struct CloakSettings {
    pub reg: bool,
    pub files: bool,
    pub kill: bool,
    pub drivers: bool,
}

/// Trait for platform-specific cloaking functionality.
pub trait VMPlatform {
    /// Method to cloak the platform based on settings.
    fn cloak(&self, settings: CloakSettings);
}

/// Enum representing the supported operating systems.
pub enum OSType {
    Unix,
    Windows,
}

/// Function to detect the operating system.
pub fn detect_os() -> OSType {
    #[cfg(target_os = "windows")]
    {
        OSType::Windows
    }
    #[cfg(not(target_os = "windows"))]
    {
        OSType::Unix
    }
}

/// Generate a random string of given length.
pub fn generate_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Kill an arbitrary process by name.
pub fn kill_processes_by_name(process_names: Vec<&str>) {
    let mut system = System::new_all();
    system.refresh_all();

    for name in process_names {
        let processes: Vec<_> = system.processes_by_name(OsStr::new(name)).collect();

        if processes.is_empty() {
            warn!("[!] {} process does not exist!", name);
        } else {
            for process in processes {
                let pid = process.pid();

                #[cfg(target_os = "windows")]
                {
                    if Command::new("taskkill")
                        .args(&["/F", "/PID", &pid.to_string()])
                        .status()
                        .is_ok()
                    {
                        info!("[*] {} process killed!", name);
                    } else {
                        warn!("[!] Failed to kill {} process!", name);
                    }
                }

                #[cfg(target_os = "linux")]
                {
                    if Command::new("kill")
                        .args(&["-9", &pid.to_string()])
                        .status()
                        .is_ok()
                    {
                        info!("[*] {} process killed!", name);
                    } else {
                        warn!("[!] Failed to kill {} process!", name);
                    }
                }
            }
        }
    }
}

/// Check if processes are running by name.
pub fn are_processes_running(process_names: Vec<&str>) -> bool {
    let mut system = System::new_all();
    system.refresh_all();

    for name in process_names {
        let processes: Vec<_> = system.processes_by_name(OsStr::new(name)).collect();

        if !processes.is_empty() {
            return true;
        }
    }

    false
}

/// Obfuscate a file or directory by renaming it.
pub fn obfuscate_file<P: AsRef<Path>>(path: P) {
    let original_path = path.as_ref();

    if original_path.exists() {
        // Generate a random string for the new name
        let random_name = if original_path.is_dir() {
            // For directories, don't use an extension
            generate_random_string(10)
        } else {
            // For files, include the extension
            format!(
                "{}.{}",
                generate_random_string(10),
                original_path
                    .extension()
                    .and_then(OsStr::to_str)
                    .unwrap_or("")
            )
        };

        let new_name = original_path.with_file_name(random_name);

        // Rename the file or directory
        match fs::rename(original_path, new_name.clone()) {
            Ok(_) => {
                info!(
                    "Renamed: {} to {}",
                    original_path.display(),
                    new_name.display()
                );
            }
            Err(e) => {
                error!("Failed to rename {}: {}", original_path.display(), e);
            }
        }
    } else {
        error!("Path does not exist: {}", original_path.display());
    }
}

/// Obfuscate entries from Windows Registry.
#[cfg(target_os = "windows")]
pub fn obfuscate_registry_entries(reg_paths: &[(&str, &str)]) {
    for (path, key) in reg_paths {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        match hklm.open_subkey_with_flags(path, KEY_SET_VALUE | KEY_QUERY_VALUE) {
            Ok(subkey) => {
                info!("[*] Renaming Reg Key {}\\{}...", path, key);
                let new_value = generate_random_string(10);
                if let Err(e) = subkey.set_value(key, &new_value) {
                    warn!("[!] Failed to set new value for {}\\{}: {:?}", path, key, e);
                }
            }
            Err(_) => {
                warn!(
                    "[!] Reg Key {}\\{} does not seem to exist! Skipping this one...",
                    path, key
                );
            }
        }
    }
}

/// Remove a specific value from the specified registry key
#[cfg(target_os = "windows")]
pub fn remove_registry_value(key_path: &str, value_name: &str) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(run_key) = hklm.open_subkey_with_flags(key_path, KEY_SET_VALUE) {
        match run_key.delete_value(value_name) {
            Ok(_) => info!("[*] Removed {} from {} registry.", value_name, key_path),
            Err(_) => warn!(
                "[!] {} key does not seem to exist in {}!",
                value_name, key_path
            ),
        }
    } else {
        warn!("[!] Failed to open {} registry key!", key_path);
    }
}

/// Remove all values from the specified registry key
#[cfg(target_os = "windows")]
pub fn remove_registry_key(key_path: &str) {
    match RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(key_path, KEY_SET_VALUE) {
        Ok(key) => {
            let result = key.delete_subkey("");
            if result.is_ok() {
                info!("Successfully deleted registry key: {}", key_path);
            } else {
                let err = result.unwrap_err();
                error!("Failed to delete registry key {}: {}", key_path, err);
            }
        }
        Err(e) => {
            error!("Failed to open registry key {}: {}", key_path, e);
        }
    }
}
