use crate::todo::Todo;
use anyhow::Result;
use md5::{Digest, Md5};
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

/// Gets a Vector of all todos that can be found in markdown-files in directorys
/// beneath the given Path.
pub fn load_todos_from_dir(dir: &Path) -> Result<Vec<Todo>> {
    let mut todos: Vec<Todo> = vec![];
    walk_folder_tree(dir, &mut todos)?;
    Ok(todos)
}

/// Recrusivly walk the directory tree and read all markdown files
fn walk_folder_tree(dir: &Path, todos: &mut Vec<Todo>) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                walk_folder_tree(&entry_path, todos)?;
            } else {
                match entry.file_name().to_str() {
                    Some(file_name) => {
                        if file_name.ends_with("md") {
                            read_file(&entry, todos)?;
                        }
                    }
                    None => panic!("Cant get filename of path {}", entry_path.to_str().unwrap()),
                }
            }
        }
    }
    Ok(())
}

/// Parses a file and builds Todo objects for each todo found
fn read_file(file: &DirEntry, todos: &mut Vec<Todo>) -> Result<()> {
    let file_content = fs::read_to_string(file.path())?;

    for (line_no, line) in file_content.lines().enumerate() {
        let line = line.trim_start();

        if !line.starts_with("- [x]") && !line.starts_with("- [ ]") {
            continue;
        }

        let todo = build_todo(file, &file_content, line, line_no)?;
        todos.push(todo);
    }

    Ok(())
}

/// Builds a todo out of the metadata of the file
fn build_todo(file: &DirEntry, file_text: &str, line: &str, line_no: usize) -> Result<Todo> {
    let name = line[6..line.len()].to_string();
    let done = line.starts_with("- [x]");
    let filename = file.file_name().to_str().unwrap().to_lowercase();
    let filepath = file.path();
    let headings = get_headings(file_text, (line_no + 1) as u32)?;

    let mut hasher = Md5::new();
    hasher.update(file_text);
    let file_md5 = hasher.finalize();

    let todo = Todo::new(
        &name,
        &filename,
        line_no + 1,
        done,
        filepath,
        headings,
        file_md5.to_vec(),
    );

    Ok(todo)
}

/// Gets all headings above the todo in reverse order.
/// First heading above the todo = first entry in vec.
fn get_headings(file_text_content: &str, todo_line_no: u32) -> Result<Vec<String>> {
    let mut headings = vec![];

    let mut line_no = 1;
    for line in file_text_content.lines() {
        if line_no == todo_line_no {
            break;
        }

        let line = line.trim_start();

        if line.starts_with('#') {
            headings.push(line.to_string());
        }

        line_no += 1;
    }
    headings.reverse();
    Ok(headings)
}
