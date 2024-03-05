use std::{fs, io};

use crate::utils::common;

pub fn check_and_increment_version() -> Result<(), Box<dyn std::error::Error>> {
    if !common::is_path_exist("package.json") {
        eprintln!("package.json not found");
        return Ok(());
    }

    let file = fs::read_to_string("package.json").unwrap();
    let mut package: serde_json::Value = serde_json::from_str(&file).expect("Error parsing JSON");

    if package.get("version").is_none() {
        package["version"] = serde_json::Value::String("1.0.0".to_string());
    }

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

    fs::write("package.json", updated_json.clone()).expect("Error writing JSON file");

    println!(
        "Updated package.json: {:?}",
        updated_json.trim().replace('\n', "")
    );
    Ok(())
}
