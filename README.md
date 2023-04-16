# hmm - capture fleeting thoughts from the terminal

## Overview

`hmm` is a simple CLI tool written in Rust that allows you to capture your thoughts and ideas in the terminal. With `hmm`, you can quickly jot down a new thought, list all your thoughts, remove a thought by its ID, or clear all your thoughts. Your thoughts are saved in a CSV file, which you can later export and share with others.

## Installation

To install `hmm`, you need to have Rust and Cargo installed on your system. If you don't have Rust and Cargo installed, you can download them from the [official Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, you can install `hmm` by running the following command:

`cargo install --git https://github.com/acrucetta/hmm.git`

After `hmm` is installed, you can run it from anywhere in your terminal by typing `hmm` followed by a command.

## Usage

`hmm` supports four commands: `add`, `ls`, `rm`, and `clear`.

### Add

The `add` command allows you to add a new thought to your list of thoughts. To add a new thought, use the following syntax:

`hmm add "My new thought"`

This will add a new thought with the content "My new thought" and the tag "personal" to your list of thoughts.

### List

The `ls` command allows you to list all your thoughts. To list all your thoughts, simply type:

`hmm ls`

```
❯ hmm ls test

#83, 2023-04-16, test
------------------------@_'-'
This is a test thought

#84, 2023-04-16, test
------------------------@_'-'
This is a test thought 2
```


This will display a list of all your thoughts, along with their IDs and tags.

### Remove

The `rm` command allows you to remove a thought by its ID. To remove a thought, use the following syntax:

`hmm rm 1`

This will remove the thought with the ID 1 from your list of thoughts.

### Clear

The `clear` command allows you to remove all your thoughts. To clear all your thoughts, simply type:

`hmm clear`


This will remove all your thoughts from the CSV file.

## Export

To export your thoughts to a CSV file, simply copy the `thoughts.csv` file to a new location on your system. You can then open the CSV file in a spreadsheet program like Microsoft Excel or Google Sheets.

## License

`hmm` is released under the MIT License. See [LICENSE](LICENSE) for details.

