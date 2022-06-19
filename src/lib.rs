use anyhow::Result;
use std::path::Path;
use todo::Todo;

pub mod error;
mod md_reader;
mod md_writer;
pub mod todo;

pub fn get_todos_from_path(dir: &Path) -> Result<Vec<Todo>> {
    md_reader::load_todos_from_dir(dir)
}

pub fn toggle_todo(todo: &mut Todo) -> Result<()> {
    md_writer::toggle_todo(todo)
}
