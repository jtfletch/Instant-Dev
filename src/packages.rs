use std::{fs, process::Command};
use toml::Value;

pub fn packages() {
    // Read the list of packages from the TOML file
    let toml_content =
        fs::read_to_string("src/packages.toml").expect("Failed to read packages.toml");
    let toml: Value = toml::from_str(&toml_content).expect("Failed to parse TOML");

    if let Some(packages) = toml.get("packages").and_then(Value::as_array) {
        // Check and install packages
        for package in packages {
            if let Some(package_name) = package.as_str() {
                if !is_installed(package_name) {
                    install_package(package_name);
                } else {
                    println!("{} is already installed.", package_name);
                }
            }
        }
    } else {
        eprintln!("Invalid or missing 'packages' array in packages.toml");
    }

    if let Some(casks) = toml.get("casks").and_then(Value::as_array) {
        // check and install casks
        for cask in casks {
            if let Some(cask_name) = cask.as_str() {
                if !is_installed(cask_name) {
                    install_cask(cask_name);
                } else {
                    println!("{} is already installed.", cask_name);
                }
            }
        }
    } else {
        eprintln!("Invalid of missing 'casks' array in packages.toml");
    }
}

pub fn is_installed(package: &str) -> bool {
    let output = Command::new("brew")
        .args(&["list", package])
        .output()
        .expect("Failed to check if package is installed.");

    output.status.success()
}

pub fn install_package(package: &str) {
    println!("Installing {}...", package);

    let output = Command::new("brew")
        .args(&["install", package])
        .output()
        .expect("Failed to install package.");

    if output.status.success() {
        println!("{} installed successfully.", package);
    } else {
        eprintln!("Failed to install {}.", package);
    }
}

pub fn install_cask(cask: &str) {
    println!("Installing {}...", cask);

    let output = Command::new("brew")
        .args(&["install", "--cask", cask])
        .output()
        .expect("Failed to install cask.");

    if output.status.success() {
        println!("{} installed successfully.", cask);
    } else {
        eprintln!("Failed to install {}.", cask);
    }
}
