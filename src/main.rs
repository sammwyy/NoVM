use clap::Parser;
use clap_derive::{Parser, ValueEnum};

use novm_core::{CloakSettings, VMPlatform};
use novm_vmware::{get_vmware, is_vmware};
use tracing::info;
use tracing_subscriber;

#[derive(Parser)]
#[clap(
    name = "NoVM",
    version = "1.0",
    about = "A tool to hide virtual machines (VMs) from malicious actors."
)]
struct Cli {
    /// Specifies the platform to cloak: 'auto', 'vmware', or 'virtualbox'
    #[clap(short, long, default_value = "auto")]
    platform: Platform,

    /// Do not cloak registry entries
    #[clap(long)]
    no_reg: bool,

    /// Do not obfuscate files
    #[clap(long)]
    no_files: bool,

    /// Do not kill processes
    #[clap(long)]
    no_kill: bool,

    /// Do not obfuscate drivers
    #[clap(long)]
    no_drivers: bool,
}

#[derive(ValueEnum, Clone)]
enum Platform {
    Auto,       // Auto-detect the platform
    Vmware,     // Force use of VMware
    Virtualbox, // Force use of VirtualBox
}

fn detect_platform() -> Option<Box<dyn VMPlatform>> {
    if is_vmware() {
        return Some(get_vmware());
    }

    None
}

fn get_platform(arg: Platform) -> Option<Box<dyn VMPlatform>> {
    match arg {
        Platform::Auto => detect_platform(),
        Platform::Vmware => Some(get_vmware()),
        _ => None,
    }
}

fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    info!("Starting NoVM...");

    let args = Cli::parse();

    // Detect platform based on user input or auto-detection
    let platform: Option<Box<dyn VMPlatform>> = get_platform(args.platform);

    let settings = CloakSettings {
        reg: !args.no_reg,
        files: !args.no_files,
        kill: !args.no_kill,
        drivers: !args.no_drivers,
    };

    // Perform the cloaking if a platform is detected
    if let Some(platform) = platform {
        info!("Cloaking virtual machine...");
        platform.cloak(settings);
    } else {
        info!("No virtual machine detected.");
    }

    info!("VM Cloaker finished.");
}
