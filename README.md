# Innocence Keeper

A tool to keep your innocence through deleting the NSFW content or files that you configured from your computer.

Known as "要留清白在人间" in Chinese, which means "Leave a clean reputation in the world".

## Installation

See release page.

## Usage

```bash
This tool removes objects from the file system rather than sending them to the recycle bin. However, for legal compliance reasons, it does not overwrite the disk, so there is still a possibility of data recovery.

Usage: innocence_keeper <COMMAND>

Commands:
  check       Check the existence of the files
  run         Run the removal of the files
  add         Add a file to the list
  remove      Remove a file from the list
  completion  Generate completion script
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Example:

```bash
innocence_keeper add /path/to/file
innocence_keeper add /path/to/directory
innocence_keeper remove /path/to/file
innocence_keeper check  // This command will check the existence of the files you added above
innocence_keeper run    // This command will remove the files you added above
```

## Note

This tool is only for educational purposes. Please use it legally and responsibly.

This tool removes objects from the file system rather than sending them to the recycle bin. However, for legal compliance reasons, it does not overwrite the disk, so there is still a possibility of data recovery.

Last but the most important, using this tool means that you agree to take full responsibility for any legal consequences that may arise from using this tool, including but not limited to the loss of data or damage to the file system, illegal usage, etc.
