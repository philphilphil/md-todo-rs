use std::fmt::{Display, Formatter, Result};
use std::path::PathBuf;

#[derive(Default, Debug)]
/// test
///
pub struct Todo {
    pub name: String,
    pub filename: String,
    pub line_no: u32,
    pub done: bool,
    pub filepath: PathBuf,
    file_md5: String,
    pub headings: Vec<String>,
}

impl Todo {
    pub fn new(
        name: &str,
        filename: &str,
        line_no: u32,
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
}

impl Todo {
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
