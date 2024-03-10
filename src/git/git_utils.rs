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

fn get_current_branch_name() -> Option<String> {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

pub fn push_to_origin() -> (u8, String) {
    // TODO:
    // 1. take the current branch name
    // 2. confirm before pushing
    let current_branch_name = get_current_branch_name();
    if current_branch_name.is_none() {
        return (0, "Current branch not found".to_string());
    }

    let current_branch_name = current_branch_name.unwrap();

    println!("Current branch: {:?}", current_branch_name);

    let output = Command::new("git")
        .args(["push", "-u", "origin", &current_branch_name])
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        (
            1,
            format!("Successfully pushed [{}] to origin", current_branch_name),
        )
    } else {
        (
            0,
            format!(
                "Error pushing to origin: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        )
    }
}
