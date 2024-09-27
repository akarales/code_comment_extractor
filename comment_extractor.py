import os
import re
import argparse
import shutil

def copy_with_comments(file_path):
    # Extracting the file name without extension
    base_name, extension = os.path.splitext(file_path)
    
    # Create the .comments directory if it doesn't exist
    comments_dir = ".comments"
    if not os.path.exists(comments_dir):
        os.makedirs(comments_dir)
    
    # File for copying the original file with comments to .comments folder
    comments_file_path = os.path.join(comments_dir, f"{os.path.basename(base_name)}_comments{extension}")
    
    # Copy the file with comments to the .comments folder
    shutil.copy(file_path, comments_file_path)
    
    print(f"Copied the original file with comments to {comments_file_path}")
    return comments_file_path

def remove_comments(file_path):
    # Regular expression for matching comments (single line and multi-line for different languages)
    comment_pattern = re.compile(r'//.*|/\*[\s\S]*?\*/|#.*')

    with open(file_path, 'r') as original_file:
        lines = original_file.readlines()
    
    new_lines = []
    for line in lines:
        # Remove comments using the regular expression
        new_line = comment_pattern.sub('', line).rstrip()
        if new_line.strip():  # Add the line only if there's code left after removing the comment
            new_lines.append(new_line + '\n')
    
    # Write the code back to the original file, overwriting it without comments
    with open(file_path, 'w') as original_file:
        original_file.writelines(new_lines)
    
    print(f"Comments removed from {file_path}")

def main():
    parser = argparse.ArgumentParser(description="Copy the original file with comments to the .comments folder and remove comments from the original file.")
    parser.add_argument('-f', '--file', type=str, required=True, help="The path to the code file to process.")
    
    args = parser.parse_args()

    # Copy the original file with comments to the .comments folder
    copy_with_comments(args.file)
    
    # Remove comments from the original file
    remove_comments(args.file)

if __name__ == '__main__':
    main()
