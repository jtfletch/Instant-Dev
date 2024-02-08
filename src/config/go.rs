use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use termion::color;

pub fn configure_go() {
    // Check if Go is installed using Homebrew
    let go_installed = check_go_installed();

    if go_installed {
        // Install Go Tools
        if let Err(e) = std::process::Command::new("go")
            .args(&["install", "golang.org/x/tools/cmd/goimports@latest"])
            .status()
        {
            eprintln!("Error installing goimports: {}", e);
            return;
        }

        if let Err(e) = std::process::Command::new("go")
            .args(&["install", "github.com/swaggo/swag/cmd/swag@latest"])
            .status()
        {
            eprintln!("Error installing swagger: {}", e);
            return;
        }

        // Check if ~/.zshrc exists
        let zshrc_path = Path::new(&env::var("HOME").unwrap()).join(".zshrc");
        if zshrc_path.exists() {
            // Read ~/.zshrc and check if the line exists
            let mut contains_line = false;
            if let Ok(file) = File::open(&zshrc_path) {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if line.contains("export PATH=${PATH}:`go env GOPATH`/bin") {
                            contains_line = true;
                            break;
                        }
                    }
                }
            }

            // If the line doesn't exist, append it
            if !contains_line {
                if let Ok(mut file) = fs::OpenOptions::new().append(true).open(&zshrc_path) {
                    if let Err(e) = writeln!(file, "export PATH=${{PATH}}:`go env GOPATH`/bin") {
                        eprintln!("Error writing to .zshrc: {}", e);
                    }
                }
            }
        } else {
            // Create ~/.zshrc and write the line into it
            if let Ok(mut file) = File::create(&zshrc_path) {
                if let Err(e) = writeln!(file, "export PATH=${{PATH}}:`go env GOPATH`/bin") {
                    eprintln!("Error writing to .zshrc: {}", e);
                }
            }
        }
        source_zshrc();
    }
}

fn source_zshrc() {
    println!("\n-----------------------------------------------");
    println!("Please run the following command in your shell:");
    println!(
        "{}source ~/.zshrc{}",
        color::Fg(color::Green),
        color::Fg(color::Reset)
    );
    println!("-----------------------------------------------");
}

fn check_go_installed() -> bool {
    match std::process::Command::new("brew").arg("list").output() {
        Ok(output) => {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                output_str.contains("go")
            } else {
                false
            }
        }
        Err(_) => false,
    }
}
