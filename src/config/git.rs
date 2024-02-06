use std::process::Command;

pub fn configure_git() {
    // Set git user.name
    let name_command = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.name")
        .arg("jtfletch")
        .output()
        .expect("Failed to set git user.name");

    if !name_command.status.success() {
        eprintln!("Error setting git user.name: {:?}", name_command.stderr);
    }

    // Set git user.email
    let email_command = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.email")
        .arg("jobetfletcher@gmail.com")
        .output()
        .expect("Failed to set git user.email");

    if !email_command.status.success() {
        eprintln!("Error setting git user.email: {:?}", email_command.stderr);
    }

    println!("Git configuration updated successfully!");
}
