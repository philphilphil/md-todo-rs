use crate::error::MDTodoError;
use crate::todo::Todo;
use anyhow::Result;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

/// Recrusivly walks all directories and parses the todos out of the files.
pub fn load_todos_from_dir(dir: &Path) -> Result<Vec<Todo>> {
    let mut todos: Vec<Todo> = vec![];
    walk_folder_tree(dir, &mut todos)?;
    Ok(todos)
}

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

// Not sure if this method should be here or in Todo.rs...
fn build_todo(file: &DirEntry, file_content: &str, line: &str, line_no: usize) -> Result<Todo> {
    let name = line[6..line.len()].to_string();
    let done = line.starts_with("- [x]");
    let filename = file.file_name().to_str().unwrap().to_lowercase();
    let filepath = file.path();
    let headings = get_headings(file_content, (line_no + 1) as u32)?;
    let file_md5 = "ddd".to_string();

    let todo = Todo::new(
        &name,
        &filename,
        line_no + 1,
        done,
        filepath,
        headings,
        file_md5,
    );

    Ok(todo)
}

fn get_headings(data: &str, todo_line_no: u32) -> Result<Vec<String>> {
    let mut headings = vec![];

    // TODO: Search in other direction
    // let mut line_no = 1;
    // let mut heading = String::from("");
    // for line in data.lines() {
    //     if line_no == todo_line_no {
    //         break;
    //     }

    //     if line.trim_start().starts_with('#') {
    //         heading = line.trim_start().to_string();
    //     }

    //     line_no += 1;
    // }

    Ok(headings)
}
