use super::*;
use crate::helpers::{read_lines_to_vec, write_lines};
use std::fs;
use std::path::PathBuf;

#[test]
fn test_analyse_rows_0() {
    let mut test_logger = logger::TestLogger::default();
    let lines = read_lines_to_vec("./test/vault/2023-09-01.md");
    analyse_lines(
        lines,
        &"To-Do's".to_string(),
        &mut test_logger,
    );
    assert_eq!(
        [
            "**General**",
            "- [/] Half-done ⭘",
            "",
            "**Another Section**",
            "- [ ] Delayed ⭘",
            "- [ ] Uncompleted ⭘",
            ""
        ],
        test_logger.stack.as_slice()
    );
}

#[test]
fn test_analyse_rows_1() {
    let mut test_logger = logger::TestLogger::default();
    let lines = read_lines_to_vec("./test/vault/2023-09-02.md");
    analyse_lines(
        lines,
        &"To-Do's".to_string(),
        &mut test_logger,
    );
    assert_eq!(
        [
            "**General**",
            "- [/] **Half-done** ⭘",
            "",
            "**Another Section**",
            "- [ ] Delayed ⭘",
            "- [ ] Uncompleted ⭘",
            ""
        ],
        test_logger.stack.as_slice()
    );
}

#[test]
fn test_update_file() -> io::Result<()> {
    setup_test_file("./test/vault/2023-09-03.md").ok();

    let mut test_logger = logger::TestLogger::default();
    let lines = read_lines_to_vec("./test/vault/tmp/2023-09-03.md");
    let updated_lines = analyse_lines(
        lines,
        &"To-Do's".to_string(),
        &mut test_logger,
    );

    write_lines("./test/vault/tmp/2023-09-03.md", &updated_lines)?;

    let new_lines = read_lines_to_vec("./test/vault/tmp/2023-09-03.md");
    let corrected_lines = read_lines_to_vec("./test/vault/2023-09-03-corrected.md");

    assert_eq!(new_lines, corrected_lines);
    Ok(())
}

fn setup_test_file(path: &str) -> io::Result<()> {
    let source = PathBuf::from(path);
    let mut destination = source.clone();
    destination.pop();
    destination.push("tmp");
    destination.push(source.file_name().unwrap());

    fs::remove_dir_all("./test/vault/tmp").ok();
    fs::create_dir("./test/vault/tmp").ok();
    fs::copy(source, destination)?;
    Ok(())
}
