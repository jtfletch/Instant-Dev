use open;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::{exit, Command, Stdio};

fn check_git() -> bool {
    let output = Command::new("ssh")
        .arg("-T")
        .arg("git@github.com")
        .output()
        .expect("Failed to execute command");

    let error_str = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        if error_str.contains("You've successfully authenticated") {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn configure_git_details(config: &str, value: &str) -> io::Result<()> {
    let command_result = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg(config)
        .arg(value)
        .output();

    if let Ok(output) = command_result {
        if output.status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error setting git {}: {:?}", config, output.stderr),
            ))
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to execute git config command",
        ))
    }
}

fn open_github() {
    let path = "https://github.com/settings/keys";

    match open::that(path) {
        Ok(()) => println!("Opened '{}' successfully.", path),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
    }
}

fn configure_directory(verbose: bool) -> io::Result<()> {
    // Set your SSH key file path
    let ssh_key_file = "~/.ssh/id_ed25519";

    // Set the SSH config file path
    let ssh_config_path = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?
        .join(".ssh")
        .join("config");

    // Check if the config file exists
    if !ssh_config_path.exists() {
        // Create the config file if it doesn't exist
        fs::File::create(&ssh_config_path)?;
    }

    // Open the config file for appending or creating if it doesn't exist
    let mut config_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&ssh_config_path)?;

    // Write the SSH configuration lines to the file
    writeln!(
        &mut config_file,
        "Host github.com\n  AddKeysToAgent yes\n  UseKeychain yes\n  IdentityFile {}",
        ssh_key_file
    )?;

    if verbose {
        println!("Configuration added to ~/.ssh/config successfully.");
    }

    Ok(())
}

fn start_ssh_agent(verbose: bool) -> io::Result<()> {
    let ssh_agent_command = Command::new("sh")
        .arg("-c")
        .arg("eval \"$(ssh-agent -s)\"")
        .output()
        .expect("Failed to start ssh-agent");

    if !ssh_agent_command.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Error starting ssh-agent: {:?}", ssh_agent_command.stderr),
        ));
    }

    if verbose {
        println!("SSH agent started successfully.")
    }
    Ok(())
}

fn generate_ssh_key(email: &str, verbose: bool) {
    let mut ssh_keygen_command = Command::new("ssh-keygen")
        .arg("-t")
        .arg("ed25519")
        .arg("-C")
        .arg(email)
        .arg("-N")
        .arg("")
        .stdin(Stdio::piped()) // Enable stdin
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start ssh-keygen process");

    // Feed input to the stdin of ssh-keygen
    if let Some(mut stdin) = ssh_keygen_command.stdin.take() {
        stdin.write_all(b"\n").expect("Failed to write Enter");
        stdin.write_all(b"\n").expect("Failed to write Enter");
    }

    let output = ssh_keygen_command
        .wait_with_output()
        .expect("Failed to wait for ssh-keygen process");

    if !output.status.success() {
        eprintln!("Error generating SSH key: {:?}", output.stderr);
        return;
    }

    if verbose {
        println!("SSH key generated successfully.");
    }
}

fn copy_public_key(verbose: bool) {
    let pbcopy_command = Command::new("sh")
        .arg("-c")
        .arg("pbcopy < ~/.ssh/id_ed25519.pub")
        .output();

    match pbcopy_command {
        Ok(output) => {
            if output.status.success() {
                if verbose {
                    println!("Public key copied to clipboard successfully.");
                }
            } else {
                eprintln!("Error copying public key to clipboard: {:?}", output.stderr);
                exit(1);
            }
        }
        Err(error) => {
            eprintln!("Error running pbcopy command: {}", error);
            exit(1);
        }
    }
}

fn configure_ssh(email: &str, verbose: bool) {
    generate_ssh_key(email, verbose);
    if let Err(error) = start_ssh_agent(verbose) {
        eprintln!("Error starting SSH agent: {}", error);
        // Handle the error as needed, e.g., return, panic, etc.
    } else if let Err(error) = configure_directory(verbose) {
        eprintln!("Error configuring directory: {}", error);
        // Handle the error as needed, e.g., return, panic, etc.
    } else {
        if verbose {
            println!("SSH configuration generated successfully.");
        }
    }
    copy_public_key(verbose);
    open_github();
}

fn set_details(user: &str, email: &str) -> io::Result<()> {
    configure_git_details("user.name", user)?;
    configure_git_details("user.email", email)?;
    Ok(())
}

pub fn configure_git(verbose: bool) {
    let user: &str = "jtfletch";
    let email: &str = "jobetfletcher@gmail.com";

    if check_git() {
        println!("Github access is already configured");
    } else {
        println!("Configuring Github access");
        match set_details(user, email) {
            Ok(_) => println!("Git details applied successfully."),
            Err(err) => eprintln!("Error configuring git: {}", err),
        }
        configure_ssh(email, verbose);
    }
}
