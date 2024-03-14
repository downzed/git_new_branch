> playing with rust to automate borring tasks

# rustomate: An Experimental Git Workflow Automator

rustomate is an experimental command-line utility developed in Rust, crafted to enhance and automate specific Git workflows. Its design focuses on simplifying the branch creation process and integrating version management for projects with a `package.json` at the root. Aimed at developers looking to refine their Git operations, rustomate is a testament to the potential for efficiency and innovation in workflow automation.

## Key Features

- **Git Repository Validation**: Ensures operation within a Git repository by verifying the presence of a `.git` directory.
- **Branch Creation**: Facilitates the creation of new Git branches based on user input.
- **Version Management**: Detects `package.json` files and guides the user through the version update process.
- **Interactive User Experience**: Utilizes interactive prompts to confirm user intentions and streamline operations.

## Prerequisites

- **Git**: Must be installed and accessible through the command line.
- **Rust and Cargo**: Required for compiling the project.

## Installation

To get started with rustomate, clone the repository and build the project using Cargo:

```bash
git clone https://github.com/downzed/rustomate
cd rustomate
cargo install --path .
```

## How to Use

Launch rustomate by specifying the desired branch name as an argument:

```bash
cargo run -- <new_branch_name>
# or, for global usage
rustomate <new_branch_name>
```

Interactive prompts will guide you through creating a new branch and, if applicable, updating the version in `package.json`.

### Example Command

```bash
cargo run -- new feature/new-feature
# or
rustomate new feature/new-feature
```

This command initiates the creation of a `feature/new-feature` branch, and if a `package.json` is found, it will prompt for a version update.

## Contributions

Your contributions to rustomate are welcome, especially those that enhance its experimental features or address potential issues. Feel free to open an issue or submit a pull request.

## License

rustomate is made available under the [MIT License](LICENSE), encouraging open and collaborative development.

## Support

If you encounter any problems or have suggestions, please file an issue on our GitHub repository.

## Future Enhancements

- **Feature Expansion**: Introducing new functionalities, accessible through distinct command-line arguments.
- **UI Improvements**: Enhancing the CLI interface for a better user experience.
- **Colorful Output**: Implementing colorful output for improved readability and engagement.
- **Error Handling**: Developing more sophisticated error handling mechanisms for enhanced reliability.
- **Testing**: Establishing a comprehensive test suite to ensure the utility's stability and functionality.
- **Change Documentation**: Maintaining a changelog to document and communicate changes effectively.

**Note**: As rustomate is an experimental tool tailored for specific workflows, it is continuously under development. Features and functionalities may evolve to better meet workflow needs.