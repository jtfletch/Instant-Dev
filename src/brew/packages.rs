use std::{fs, process::Command};
use termion::color;
use toml::Value;

/// Retrieves a list of installed packages using Homebrew.
fn get_installed_packages() -> Vec<String> {
    let output = Command::new("brew")
        .args(&["list"])
        .output()
        .expect("Failed to get list of installed packages.");

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.trim().to_string())
            .collect()
    } else {
        eprintln!("Failed to get list of installed packages.");
        Vec::new()
    }
}

/// Retrieves a list of installed casks using Homebrew.
fn get_installed_casks() -> Vec<String> {
    let output = Command::new("brew")
        .args(&["list", "--cask"])
        .output()
        .expect("Failed to get list of installed casks.");

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.trim().to_string())
            .collect()
    } else {
        eprintln!("Failed to get list of installed casks.");
        Vec::new()
    }
}

/// Installs a package using Homebrew.
fn install_package(package: &str) {
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

/// Installs a cask using Homebrew.
fn install_cask(cask: &str) {
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

/// Installs packages and casks listed in the `packages.toml` file using Homebrew.
pub fn packages() {
    println!(
        "\n{} --- Installing Packages with Homebrew --- {}",
        color::Fg(color::Yellow),
        color::Fg(color::Reset)
    );

    // Get the username using `echo $USER` command
    let username_output = Command::new("sh")
        .arg("-c")
        .arg("echo $USER")
        .output()
        .expect("Failed to get username.");

    let username = String::from_utf8_lossy(&username_output.stdout)
        .trim()
        .to_string();

    let toml_path = format!("/Users/{}/.config/instant-dev/packages.toml", username);

    if !fs::metadata(&toml_path).is_ok() {
        println!("No packages.toml file found. Skipping...");
        return;
    }

    let toml_content =
        fs::read_to_string(&toml_path).expect("Failed to read packages.toml");

    let toml: Value = toml::from_str(&toml_content).expect("Failed to parse TOML");

    let installed_packages = get_installed_packages();

    if let Some(packages) = toml.get("packages").and_then(Value::as_array) {
        // Check and install packages
        for package in packages {
            if let Some(package_name) = package.as_str() {
                let package_name = package_name.to_string();
                if !installed_packages.contains(&package_name) {
                    install_package(&package_name);
                } else {
                    println!("{} is already installed.", package_name);
                }
            }
        }
    } else {
        eprintln!("Invalid or missing 'packages' array in packages.toml");
    }

    let installed_casks = get_installed_casks();

    if let Some(casks) = toml.get("casks").and_then(Value::as_array) {
        // Check and install casks
        for cask in casks {
            if let Some(cask_name) = cask.as_str() {
                let cask_name = cask_name.to_string();
                if !installed_casks.contains(&cask_name) {
                    install_cask(&cask_name);
                } else {
                    println!("{} is already installed.", cask_name);
                }
            }
        }
    } else {
        eprintln!("Invalid or missing 'casks' array in packages.toml");
    }
}
