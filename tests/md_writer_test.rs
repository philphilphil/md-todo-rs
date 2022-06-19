mod common;
use common::md_test_file_creator;
use md_todo::{md_reader, md_writer};
use tempfile::TempDir;

#[test]
fn test_tick_single_file() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_1_open_todo(&dir, "file0.md").unwrap();

    let mut todos = md_reader::load_todos_from_dir(dir.path()).unwrap();
    assert!(!todos[0].done);

    md_writer::toggle_todo(&mut todos[0]).unwrap();
    let todos = md_reader::load_todos_from_dir(dir.path()).unwrap();
    assert!(todos[0].done);

    dir.close().unwrap();
}

#[test]
fn test_parse_and_tick_multiple_files_multiple_todos() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir, "file_123_323.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir, "afile_123_323.md").unwrap();

    let mut todos = md_reader::load_todos_from_dir(dir.path()).unwrap();
    assert_eq!(todos.len(), 10);
    assert_eq!(todos[0].name, "A open todo!");
    assert!(!todos[1].done);
    assert!(todos[2].done);

    let mut open_todos_count = todos.iter().filter(|t| !t.done).count();
    assert_eq!(open_todos_count, 8);

    md_writer::toggle_todo(&mut todos[1]).unwrap();
    // need to reload to get newest md5 since we are on the same file
    let mut todos = md_reader::load_todos_from_dir(dir.path()).unwrap();
    md_writer::toggle_todo(&mut todos[2]).unwrap();
    let todos = md_reader::load_todos_from_dir(dir.path()).unwrap();
    assert!(todos[1].done);
    assert!(!todos[2].done);
    open_todos_count = todos.iter().filter(|t| !t.done).count();
    assert_eq!(open_todos_count, 8); // should not change because changed one done one open

    dir.close().unwrap();
}

#[test]
#[should_panic]
fn test_file_changed() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_1_open_todo(&dir, "file0.md").unwrap();

    let mut todos = md_reader::load_todos_from_dir(dir.path()).unwrap();
    assert!(!todos[0].done);

    // toggle twice without reloading, this should panic since the file changed.
    md_writer::toggle_todo(&mut todos[0]).unwrap();
    md_writer::toggle_todo(&mut todos[0]).unwrap();

    dir.close().unwrap();
}
