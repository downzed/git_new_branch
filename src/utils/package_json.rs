use std::{fs, io};

use crate::utils::common;

pub fn create_package_json(package: &mut serde_json::Value) {
    let version = package["version"].as_str().unwrap();

    println!("Current version: {:?}", version);

    common::verify_user_confirm("Would you like to change version? [Y/n]", "");

    println!("New version number:");
    let mut new_version_number = String::new();
    io::stdin()
        .read_line(&mut new_version_number)
        .expect("Failed to read line");

    package["version"] = serde_json::Value::String(new_version_number.trim().to_string());

    let updated_json = serde_json::to_string_pretty(&package).expect("Error serializing JSON");

    println!(
        "Updating package.json: {:?}",
        updated_json.trim().replace('\n', "")
    );

    common::verify_user_confirm("Are you sure [Y/n]?", "");
    let updated_json = serde_json::to_string_pretty(&package).expect("Error serializing JSON");

    fs::write("package.json", updated_json).expect("Error writing JSON file");
}

pub fn update() {
    if common::is_path_exist("package.json") {
        let file = fs::read_to_string("package.json").unwrap();

        let mut package: serde_json::Value =
            serde_json::from_str(&file).expect("Error parsing JSON");

        if package.get("version").is_none() {
            println!("package.json version not found");
            return;
        }

        create_package_json(&mut package);
    }
}
