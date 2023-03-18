# thg

thg is a command-line tool for capturing your thoughts in the terminal. It allows you to add, list, and remove thoughts with simple commands.

## Installation

To install thg, you'll need to have Rust installed on your system. Once you have Rust installed, you can install thg using Cargo:

```bash
cargo install thg
```

### Usage
Here are the available commands and their usage:

```bash
thg + [-t TAGS]   # Add a new thought
thg ls            # List all thoughts
thg rm ID       # Remove a thought by ID
```
To add a new thought, use the add command. You'll be prompted to enter your thought, and you can optionally add tags by using the -t or --tags flag.

To list all thoughts, use the list command.

To remove a thought, use the remove command followed by the ID of the thought you want to remove.