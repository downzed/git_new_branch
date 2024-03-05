use std::process::Command;

use crate::utils::common;

pub fn check_for_git_root() -> u8 {
    if !common::is_path_exist(".git") {
        eprintln!(".git repo not found. Please clone or init it first.");
        println!("\n >> https://docs.github.com/en/get-started");
        return 0;
    }
    1
}

pub fn create_branch(name: &str) -> (u8, String) {
    let output = Command::new("git")
        .args(["checkout", "-b", name])
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        (
            1,
            format!("Successfully checked out to new branch: {}", name),
        )
    } else {
        (
            0,
            format!(
                "Error checking out to new branch: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        )
    }
}
