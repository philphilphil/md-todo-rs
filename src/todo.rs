use std::fmt::{Display, Formatter, Result};
use std::path::PathBuf;

#[derive(Default, Debug)]
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
    pub file_md5: String,
    /// All headings above the todo in reverse order
    pub headings: Vec<String>,
}

impl Todo {
    /// Creates a new todo
    pub fn new(
        name: &str,
        filename: &str,
        line_no: usize,
        done: bool,
        filepath: PathBuf,
        headings: Vec<String>,
        file_md5: String,
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
    ///
    /// # Examples
    ///
    /// ```
    /// //let todo = Todo::Default();
    /// //todo.toggle();
    /// ```
    pub fn toggle(&mut self) {
        // md_writer::toggle_todo(self).unwrap();
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
