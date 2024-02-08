use std::io::{self};
use std::process::{Command, exit};
use termion::color;

/// Checks if Homebrew is installed.
fn is_homebrew_installed() -> bool {
    Command::new("brew").arg("--version").output().is_ok()
}

/// Updates Homebrew to the latest version.
///
/// # Errors
///
/// This function returns an error if the update process fails.
fn update_homebrew() -> Result<(), io::Error> {
    let output = Command::new("brew").arg("update").output()?;
    if output.status.success() {
        let update_output = String::from_utf8_lossy(&output.stdout);
        if update_output.contains("Already up-to-date.") {
            println!("Homebrew is already at the latest release version.");
        } else {
            println!("Homebrew updated.");
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

/// Installs Homebrew if it is not already installed, and updates it if it is.
///
/// # Errors
///
/// This function returns an error if there are any issues with the installation or update process.
///
/// # Example
///
/// ```
/// use std::io;
/// use my_crate::install_homebrew;
///
/// fn main() -> Result<(), io::Error> {
///     install_homebrew()?;
///     Ok(())
/// }
/// ```
pub fn install_homebrew() -> Result<(), io::Error> {
    println!("\n{}--- Homebrew Installation and Configuration ---{}",
        color::Fg(color::Yellow),
        color::Fg(color::Reset)
    );
    if is_homebrew_installed() {
        println!("Homebrew is already installed, checking for updates...");
        // Perform any necessary post-installation tasks
        update_homebrew()?;
    } else {
        println!("\n{}-------------------------------------------------------------------------------------------------",
            color::Fg(color::Green)
        );
        println!("Homebrew is not installed.");
        println!("Please install Homebrew using the following command:");
        println!("{}/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"{}",
            color::Fg(color::Cyan),
            color::Fg(color::Green)
        );
        println!("Then re-run this application.");
        println!("-------------------------------------------------------------------------------------------------{}",
            color::Fg(color::Reset)
        );
        exit(0); // Exit the application with an error code
    }

    println!("Homebrew installation and configuration completed successfully.");
    Ok(())
}
