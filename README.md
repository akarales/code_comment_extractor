# Python Comment Remover and Backup

This Python script copies the original file (with all comments) into a `.comments` folder and removes all comments from the original file, leaving only the code.

## Features

- **Backup Original File**: A copy of the original file with all comments is created in a `.comments` folder.
- **Remove Comments**: The original file is modified, and all comments are removed, leaving only the code.

## Requirements

- Python 3.x

## Installation

You can install this script as a system-wide application so that it can be run globally from the terminal.

### Step 1: Move or Symlink the Script to a Directory in Your `PATH`

You need to place the Python script in a directory that is included in your system's `PATH`, such as `/usr/local/bin`.

```bash
# First, make the script executable
chmod +x comment_remover.py

# Option 1: Move the script to /usr/local/bin
sudo mv comment_remover.py /usr/local/bin/comment_remover

# After this, you can run the script globally as:
comment_remover -f your_file.rs

# Option 2: Create a Symlink to /usr/local/bin
# If you prefer not to move the script, you can create a symbolic link instead:
sudo ln -s /path/to/your/script/comment_remover.py /usr/local/bin/comment_remover