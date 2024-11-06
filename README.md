# my-wc-command using rust
implementation of wc command using rust


The `wc` (word count) command is a simple yet powerful utility in Unix/Linux that is used to count the number of lines, words, and characters this only work for files and doesn't support standard input

## Functionality

The `wc` command can be used to count:

- **Lines**: The number of lines in a file using.
- **Words**: The number of words in a file.
- **Characters**: The number of characters in a file (or bytes if using the `-c` flag).
- **Bytes**: The total number of bytes in the file.

## Basic Syntax

```bash
wc [flag] [file...]


# Count lines, words, and characters in a file
 wc filename.txt

# Count only the number of lines in a file
wc -l filename.txt

# Count only the number of words in a file
wc -w filename.txt

# Count only the number of characters (bytes) in a file
wc -c filename.txt

# Count only the number of characters (characters, not bytes) in a file
wc -m filename.txt

