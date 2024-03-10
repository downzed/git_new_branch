use clap::{arg, Arg, ArgMatches, Command};

use crate::utils::{common, package_json};

fn cli() -> Command {
    // make it blue
    let about = "A fictional automate CLI tool";

    Command::new("rustomate")
        .version("0.1.0")
        .author("Ziv Zerr <downzed@gmail.com>")
        .about(about)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("new")
                .about("Create a new branch")
                .arg(arg!(<BRANCH_NAME> "The new branch to create"))
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("po")
                .about("Push to origin")
                .alias("pushtoorigin")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .help("origin name, default: 'origin main'"),
                )
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("up")
                .about("Update package.json")
                .alias("update")
                .arg_required_else_help(false), // .arg(
                                                //     arg!(youpee: -U --up)
                                                //         .action(ArgAction::SetTrue)
                                                //         .help("Push to origin after updating package.json"),
                                                // ),
        )
}

fn handle_matches(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("po", _)) => {
            println!("Pushing to origin");

            // TODO:
            // 1. take the current branch name
            // 2. confirm before pushing
        }

        Some(("up", _)) => {
            println!("Updating package.json");
            // TODO: update package.json
            package_json::update();
        }

        Some(("new", sub_matches)) => {
            if let Some(branch_name) = sub_matches.get_one::<String>("BRANCH_NAME") {
                println!("Creating new branch: {}", branch_name);
                common::verify_user_confirm(
                    format!(
                        "This will create branch '{}',\n are you sure [Y/n]? ",
                        branch_name
                    )
                    .as_str(),
                    "Branch creation aborted.",
                );
                // TODO: create new branch
                //      git_utils::create_branch(branch_name);
                // TODO: suggest to push to origin
            }
        }
        _ => unreachable!(),
    }
}
pub fn run() {
    let matches = cli().get_matches();

    handle_matches(&matches);
}
