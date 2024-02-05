use std::process::{exit, Command};

pub fn is_brew_installed() -> Option<String> {
    let output = Command::new("brew").arg("--version").output().ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

pub fn install_homebrew() {
    if let Some(version) = is_brew_installed() {
        println!("{} is already installed.", version);

        let update_command = Command::new("brew").arg("update").output();

        if let Ok(output) = update_command {
            if output.status.success() {
                // Parse the update output and check if it contains "Already up-to-date."
                let update_output = String::from_utf8_lossy(&output.stdout);
                if update_output.contains("Already up-to-date.") {
                    println!("Homebrew is at the latest release version.");
                } else {
                    println!("Homebrew is at the latest version.");
                }
            } else {
                println!("Failed to update Homebrew.");
            }
        } else {
            println!("Failed to execute update command.");
        }
    } else {
        println!("Homebrew is not installed.");
        println!("Would you like to install it now? (y/n)");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim().to_lowercase() == "y" {
            let install_command = Command::new("bash")
                .arg("-c")
                .arg("$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)")
                .status();

            if let Ok(status) = install_command {
                if status.success() {
                    println!("Homebrew installed successfully.");

                    // Now, check for updates
                    let update_command = Command::new("brew").arg("update").status();

                    if let Ok(status) = update_command {
                        if status.success() {
                            println!("Homebrew is at the latest release version.");
                        } else {
                            println!("Failed to update Homebrew.");
                        }
                    } else {
                        println!("Failed to execute update command.");
                    }
                } else {
                    println!("Failed to install Homebrew.");
                }
            } else {
                println!("Failed to execute installation command.");
            }
        } else {
            println!("Installation canceled.");
            exit(0);
        }
    }
}
