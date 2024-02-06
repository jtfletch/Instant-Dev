use std::io::{self, Write};
use std::process::{exit, Command};

pub fn is_brew_installed() -> Option<String> {
    let output = Command::new("brew").arg("--version").output().ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

fn update_homebrew() -> Result<(), std::io::Error> {
    let output = Command::new("brew").arg("update").output()?;
    if output.status.success() {
        let update_output = String::from_utf8_lossy(&output.stdout);
        if update_output.contains("Already up-to-date.") {
            println!("Homebrew is at the latest release version.");
        } else {
            println!("Homebrew is at the latest version.");
        }
        Ok(())
    } else {
        eprintln!("Failed to update Homebrew: {:?}", output.status);
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Update failed",
        ))
    }
}

fn install_homebrew_from_script() -> Result<(), std::io::Error> {
    let status = Command::new("bash")
        .arg("-c")
        .arg("$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)")
        .status()?;
    if status.success() {
        println!("Homebrew installed successfully.");
        update_homebrew()?;
        Ok(())
    } else {
        eprintln!("Failed to install Homebrew: {:?}", status);
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Installation failed",
        ))
    }
}

pub fn install_homebrew() {
    if let Some(version) = is_brew_installed() {
        println!("{} is already installed.", version);
        if let Err(err) = update_homebrew() {
            eprintln!("Failed to update Homebrew: {:?}", err);
        }
    } else {
        println!("Homebrew is not installed.");
        println!("Would you like to install it now? (y/n)");

        let mut input = String::new();
        io::stdout().flush().unwrap(); // Flush the buffer before reading input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim().to_lowercase() == "y" {
            if let Err(err) = install_homebrew_from_script() {
                eprintln!("Failed to install Homebrew: {:?}", err);
                exit(1);
            }
        } else {
            println!("Installation canceled.");
            exit(0);
        }
    }
}
