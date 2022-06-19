use anyhow::Result;
use error::MDTodoError;
use std::path::Path;
use todo::Todo;

pub mod error;
mod md_reader;
mod md_writer;
pub mod todo;

/// Gets a Vector of all todos that can be found in markdown-files in directorys
/// beneath the given Path.
pub fn get_todos_from_path(dir: &dyn AsRef<Path>) -> Result<Vec<Todo>> {
    if !dir.as_ref().is_dir() {
        return Err(anyhow::Error::new(MDTodoError::NotADir));
    }

    md_reader::get_todos_from_dir(dir.as_ref())
}

/// Toggles the done state of the todo in the physical file.
///
/// This is done via finding the line of the todo and replacing `[ ]` with `[x]` or the other way.
/// Before the change is done, its checked if the file changed since the last load of todos.
/// If yes an error is returned.
pub fn toggle_todo(todo: &mut Todo) -> Result<()> {
    md_writer::toggle_todo(todo)
}
