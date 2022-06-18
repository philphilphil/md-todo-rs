use std::fmt::{Display, Formatter, Result};
use std::path::PathBuf;

use crate::md_writer;

#[derive(Debug, Default)]
/// A todo line in the markdown file
pub struct Todo {
    /// The text of the todo
    pub name: String,
    /// Filename of the file containing the todo
    pub filename: String,
    /// Line number where the todo is
    pub line_no: usize,
    /// Is the todo marked as done or not
    pub done: bool,
    /// Full path to the file containing the todo
    pub filepath: PathBuf,
    /// MD5 hash of the file containing the todo
    pub file_md5: Vec<u8>,
    /// All headings above the todo in reverse order
    pub headings: Vec<String>,
}

impl Todo {
    /// Creates a new todo with all its metadata
    pub fn new(
        name: &str,
        filename: &str,
        line_no: usize,
        done: bool,
        filepath: PathBuf,
        headings: Vec<String>,
        file_md5: Vec<u8>,
    ) -> Todo {
        Todo {
            name: name.to_string(),
            filename: filename.to_string(),
            line_no,
            done,
            filepath,
            headings,
            file_md5,
        }
    }

    /// Toggles the state of the todo to open or done.
    pub fn toggle(&mut self) {
        md_writer::toggle_todo(self).unwrap();
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut todo_state = "[ ]";
        if self.done {
            todo_state = "[x]"
        }
        write!(f, "{} {}", todo_state, self.name)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_simple_todo() -> Todo {
        Todo::new(
            "A todo",
            "SomeFile.md",
            33,
            false,
            PathBuf::default(),
            vec!["# One".to_string(), "# Two".to_string()],
            vec![1, 2, 3],
        )
    }

    #[test]
    fn test_creation_with_new() {
        let todo = get_simple_todo();

        assert_eq!(todo.name, "A todo");
        assert_eq!(todo.filename, "SomeFile.md");
        assert_eq!(todo.line_no, 33);
        assert!(!todo.done);
        assert_eq!(todo.filepath, PathBuf::default());
        assert_eq!(todo.headings[1], "# Two");
        assert_eq!(todo.file_md5[2], 3);
    }

    #[test]
    fn test_display() {
        let mut todo = get_simple_todo();
        assert_eq!(format!("{}", todo), "[ ] A todo");
        todo.done = true;
        assert_eq!(format!("{}", todo), "[x] A todo");
    }
}
