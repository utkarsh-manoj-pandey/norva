// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/main.rs
// Description: Application entrypoint and process supervisor.
// ============================================================================

#![allow(dead_code, unused_imports)]

use norva::{NorvaBrowser, NorvaConfig};
use std::env;

/// Displays the executive NORVA terminal banner on stdout.
///
/// I have written this banner display because launching NORVA should immediately communicate
/// its security posture: an uncompromising, lightweight, high-performance privacy shield built in pure Rust.
fn print_norva_banner(secure_mode: bool) {
    println!(
        r#"
  ███╗   ██╗ ██████╗ ██████╗ ██╗   ██╗ █████╗ 
  ████╗  ██║██╔═══██╗██╔══██╗██║   ██║██╔══██╗
  ██╔██╗ ██║██║   ██║██████╔╝██║   ██║███████║
  ██║╚██╗██║██║   ██║██╔══██╗╚██╗ ██╔╝██╔══██║
  ██║ ╚████║╚██████╔╝██║  ██║ ╚████╔╝ ██║  ██║
  ╚═╝  ╚═══╝ ╚═════╝ ╚═╝  ╚═╝  ╚═══╝  ╚═╝  ╚═╝
  Native Open-source Rust-based Virtual Access
  High-Security • Anti-Tracking • Hardware Isolated • Multi-Engine
"#
    );

    if secure_mode {
        println!("  >>> [ACTIVE] SECURE GHOST MODE (Zero-trace volatile memory session)");
    } else {
        println!("  >>> [ACTIVE] HARDENED PRIVACY SESSION (5 Isolated SQLite DBs)");
    }
    println!("  >>> Camera & Mic: Hardware Zero-Trust Arbiter Active");
    println!("  >>> Geolocation: Strict Origin Protection Active");
    println!("  >>> Search Engines: DuckDuckGo | Google | Bing | Yahoo | Ahmia (.onion) | Startpage | Brave | GitHub | Wikipedia");
    println!("  >>> Anti-Tracking: Active Surveillance Neutralization Online\n");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // I have written this CLI argument parser because power users need the ability
    // to launch NORVA directly into Secure Mode (--secure / --incognito) or navigate
    // to an initial target URL via the command line.
    let args: Vec<String> = env::args().collect();
    let mut secure_mode = false;
    let mut initial_url: Option<String> = None;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--secure" | "--incognito" | "-s" => {
                secure_mode = true;
            }
            "--help" | "-h" => {
                println!("NORVA Browser - Native Open-source Rust-based Virtual Access");
                println!("Usage: norva [OPTIONS] [URL]");
                println!("\nOptions:");
                println!("  -s, --secure, --incognito    Launch in ephemeral Secure Ghost Mode (zero disk writes)");
                println!("  -h, --help                   Display this help dialog");
                return Ok(());
            }
            url if !url.starts_with('-') => {
                initial_url = Some(url.to_string());
            }
            _ => {}
        }
    }

    // Display banner
    print_norva_banner(secure_mode);

    // I have written this configuration initialization to create POSIX-isolated profile
    // paths and prepare separate database files for each use case.
    let config = NorvaConfig::init(secure_mode);

    // Instantiate and execute the master browser runtime
    let browser = NorvaBrowser::new(config, initial_url);
    browser.run()?;

    Ok(())
}
