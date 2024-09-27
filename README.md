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
After this step, the comment_remover script will be available for use from anywhere in your terminal.

Usage
Command-Line Options
-f or --file: Specify the path to the code file to process.


#Example
Copy the Original File with Comments and Remove Comments from the Original
bash
Copy code
comment_remover -f merge_strings_alternately.rs

This will:

Copy the original file merge_strings_alternately.rs with all comments to the .comments folder as merge_strings_alternately_comments.rs.
Remove all comments from merge_strings_alternately.rs and leave only the code.
Run Help Command
To display all available options and usage instructions, run the help command:

comment_remover --help

bash
Copy code
comment_remover --help
Explanation of the Process
Backup with Comments: The script makes an exact copy of the original file with all comments included, and stores it in a .comments directory. This ensures that you always have the original version available for reference.
Remove Comments: After copying, the script removes all comments (single-line and multi-line comments) from the original file and only retains the code. It recognizes comments in C-style (//, /* ... */) and Python-style (#).
Notes
The .comments folder is created in the same directory where the script is run, and it will contain the backup copies of files with comments.
You can repeat this process for multiple files by running the script with the -f option and specifying different file paths.
Uninstallation
To uninstall the script:

bash
Copy code
# Remove the script from /usr/local/bin:
sudo rm /usr/local/bin/comment_remover

# Remove the .desktop file (if created):
sudo rm /usr/share/applications/comment_remover.desktop
Once the script is removed, it will no longer be accessible globally from the terminal or your applications menu.