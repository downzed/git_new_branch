use std::{
    fs,
    io::{self, Write},
    process,
};

pub fn is_path_exist(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn verify_user_confirm(message: &str, abort_message: &str) {
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
