use std::{
    env, fs,
    io::{self, Write},
    process::{self, Command},
};

fn main() {
    if check_for_git_root() == 0 {
        process::exit(1);
    }

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: {} <new branch name>", args[0]);
        process::exit(1);
    }

    let new_branch_name = &args[1];

    verify_user_confirm(
        format!(
            "This will create branch '{}',\n are you sure [Y/n]? ",
            new_branch_name
        )
        .as_str(),
        "Branch creation aborted.",
    );
    {
        let res = create_branch(new_branch_name);

        println!("{}", res.1);

        if res.0 != 0 {
            check_and_increment_version();
        } else {
            process::exit(1);
        }
    }
}

fn is_path_exist(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn check_for_git_root() -> u8 {
    if !is_path_exist(".git") {
        eprintln!(".git repo not found. Please clone or init it first.");
        println!("\n >> https://docs.github.com/en/get-started");
        return 0;
    }
    1
}

fn verify_user_confirm(message: &str, abort_message: &str) {
    println!("{}", message);
    io::stdout().flush().unwrap();

    let mut confirm = String::new();

    io::stdin()
        .read_line(&mut confirm)
        .expect("Failed to read line");

    if confirm.trim().to_uppercase() == "N" {
        if !abort_message.is_empty() {
            println!("{}", abort_message);
        }
        process::exit(0);
    }
}

fn check_and_increment_version() {
    if is_path_exist("package.json") {
        let file = fs::read_to_string("package.json").unwrap();
        let mut package: serde_json::Value =
            serde_json::from_str(&file).expect("Error parsing JSON");

        if package.get("version").is_none() {
            println!("package.json version not found");
            return;
        }

        let version = package["version"].as_str().unwrap();

        println!("Current version: {:?}", version);

        verify_user_confirm("Would you like to change version? [Y/n]", "");

        println!("New version number:");
        let mut new_version_number = String::new();
        io::stdin()
            .read_line(&mut new_version_number)
            .expect("Failed to read line");

        package["version"] = serde_json::Value::String(new_version_number.trim().to_string());

        let updated_json = serde_json::to_string_pretty(&package).expect("Error serializing JSON");

        println!(
            "Updated package.json: {:?}",
            updated_json.trim().replace('\n', "")
        );

        verify_user_confirm("Are you sure [Y/n]?", "");
        let updated_json = serde_json::to_string_pretty(&package).expect("Error serializing JSON");

        fs::write("package.json", updated_json).expect("Error writing JSON file");
    }
}

fn create_branch(name: &str) -> (u8, String) {
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
