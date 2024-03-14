> playing with rust to automate borring tasks

# Rustomate: Git Branch Creator & Version Updater

Rustomate is a Rust-based command-line utility tailored for enhancing Git workflows. It not only facilitates the seamless creation of new Git branches but also intelligently interacts with `package.json` files for version management. Designed with simplicity and efficiency in mind, Rustomate streamlines your development process, making Git operations more intuitive and less error-prone.

## Key Features

- **Git Repository Validation**: Automatically checks for a `.git` directory to confirm the utility is executed within a Git repository.
- **Branch Creation**: Enables the creation of a new Git branch, customized with a name of your choice.
- **Version Management**: Identifies `package.json` at the project root, offering an option to update the version number.
- **Interactive Experience**: Engages users with interactive prompts to confirm before proceeding with any significant actions.

## Prerequisites

Before diving into Rustomate, ensure you have the following prerequisites satisfied:

- Git should be installed and accessible from your command line environment.
- Rust and Cargo need to be set up on your system to facilitate building and running the utility.

## Installation

Get started with Rustomate by cloning the repository and building the utility using the following commands:

```bash
git clone https://github.com/downzed/rustomate.git
cd rustomate
cargo install --path .
```

## How to Use

Execute Rustomate with your desired new branch name as a command-line argument. You can either run it directly if installed or use Cargo:

### Running with Cargo

```bash
cargo run -- <new_branch_name>
```

### Running as an Installed Command

```bash
rustomate <new_branch_name>
```

Follow the interactive prompts to create your new branch and, if applicable, update your `package.json` version.

### Example

```bash
cargo run -- feature/new-feature
# or
rustomate feature/new-feature
```

This example demonstrates how to create a branch named `feature/new-feature`. If a `package.json` file is detected, Rustomate will also prompt for a version update.

## Contributing to Rustomate

Your contributions make the open-source community a vibrant and innovative space! If you're interested in contributing to Rustomate, feel free to open an issue or submit a pull request.

## License

Rustomate is made available under the [MIT License](LICENSE), promoting open and permissive software development.

## Support & Questions

Encountered an issue or have questions? Don't hesitate to [file an issue on GitHub](https://github.com/downzed/rustomate/issues).

## Planned Enhancements

- **Feature Expansion**: Integrate additional functionalities, each accessible via unique command-line arguments.
- **User Interface Improvements**: Enhance the CLI experience with a more user-friendly interface and colorful output.
- **Robust Error Handling**: Implement improved error management mechanisms.
- **Testing**: Incorporate tests to ensure reliability and stability.
- **Changelog**: Maintain a changelog to document the evolution of Rustomate.

Rustomate is designed for personal workflows and serves as a playground for Rust experimentation. It's more than just a utility; it's a companion for your Git adventures.
