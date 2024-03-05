use std::{env, process};

use tui::app;
use utils::{common, package_json};

use git::git_utils::{self};

mod git {
    pub mod git_utils;
}

mod utils {
    pub mod common;
    pub mod package_json;
}

mod tui {
    pub mod app;
}

fn main() {
    app::run_tui();

    if git_utils::check_for_git_root() == 0 {
        process::exit(1);
    }

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: {} <new branch name>", args[0]);
        process::exit(1);
    }

    let new_branch_name = &args[1];

    common::verify_user_confirm(
        format!(
            "This will create branch '{}',\n are you sure [Y/n]? ",
            new_branch_name
        )
        .as_str(),
        "Branch creation aborted.",
    );
    {
        let res = git_utils::create_branch(new_branch_name);

        println!("{}", res.1);

        if res.0 != 0 {
            package_json::check_and_increment_version();
        } else {
            process::exit(1);
        }
    }
}
