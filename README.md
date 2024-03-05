> playing with rust to automate borring tasks

# Git Branch Creator & Version Updater

This utility is a Rust-based command-line tool designed to streamline Git workflows. It automates the creation of a new Git branch. If a `package.json` file is present in the root, it also prompts the user to optionally update the version number within the file.

## Features

- Check for `.git` directory to ensure the command is run within a Git repository.
- Create a new Git branch with a user-defined name.
- Detect `package.json` in the project root and offer version update.
- Interactive prompts for user confirmations to proceed with actions.

## Prerequisites

- Git installed and accessible from the command line.
- Rust and Cargo installed for building the project.

## Getting Started

To use this utility, clone the repository to your local machine and build the project:

```bash

git clone https://github.com/downzed/git_new_branch
cd git_new_branch
cargo install --path .

```



## Usage

Run the utility with the desired branch name as an argument:

```bash
cargo run -- <new_branch_name>
```
-- or if installed --
```bash
git_new_branch <new_branch_name>
```

Follow the interactive prompts to create a new branch and optionally update the version in `package.json`.

## Example

```bash
cargo run -- feature/new-feature || git_new_branch feature/new-feature
```

This command will attempt to create a new branch named `feature/new-feature`. If `package.json` is found, it will also prompt to update the version.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is licensed under [MIT](LICENSE).

## Support

If you encounter any issues or have questions, please file an issue on GitHub.



## TODO:

- Add more features
    - each function in a separate command line argument
    - better cli interface
    - colorful output
- Add better error handling
- Add tests
- Add changelog

