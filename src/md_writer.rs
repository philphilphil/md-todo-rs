use crate::todo::Todo;
use std::fs;
use std::io;
use std::path::Path;

/// Toggles the done state of the todo.
/// This is done via finding the line of the todo and then replacing the markdoen
/// todo syntax. There is no other change, just a replace.
pub fn toggle_todo(todo: &mut Todo) -> io::Result<()> {
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