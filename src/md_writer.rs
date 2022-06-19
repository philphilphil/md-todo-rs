use crate::error::MDTodoError;
use crate::md_reader;
use crate::todo::Todo;
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Toggles the done state of the todo in the physical file.
/// This is done via finding the line of the todo and replacing `[ ]` with `[x]` or the other way.
/// There is no other change, just a replace.
///
/// Before the change is done, its checked if the file changed since the last load of todos.
/// If yes an error is returned.
pub fn toggle_todo(todo: &mut Todo) -> Result<()> {
    if did_file_change_since_read(todo)? {
        return Err(anyhow::Error::new(MDTodoError::FileChanged));
    }

    let data = fs::read_to_string(Path::new(&todo.filepath))?;
    let mut new_data: String = String::new();

    let mut line_no = 1;
    for line in data.lines() {
        if line_no == todo.line_no {
            if todo.done {
                new_data += &line.replace("- [x]", "- [ ]");
                todo.done = false;
            } else {
                new_data += &line.replace("- [ ]", "- [x]");
                todo.done = true;
            }
        } else {
            new_data += line;
        }
        new_data += "\n";
        line_no += 1;
    }

    fs::write(&todo.filepath, new_data)?;

    Ok(())
}

fn did_file_change_since_read(todo: &mut Todo) -> Result<bool> {
    let file_content = fs::read_to_string(&todo.filepath)?;
    let file_md5 = md_reader::get_file_content_md5(&file_content);
    Ok(file_md5 != todo.file_md5)
}
