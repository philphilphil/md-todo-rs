pub mod error;
pub mod md_reader;
pub mod todo;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::md_reader;

    #[test]
    fn it_works() {
        let todos = md_reader::load_todos_from_dir(Path::new("/Users/Phil/TestingNotes/"));
        dbg!(&todos);
        assert_eq!(todos.unwrap().len(), 16);
    }
}
