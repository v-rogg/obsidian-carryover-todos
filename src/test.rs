use super::*;
use crate::helpers::{read_lines_to_vec, write_lines};
use std::fs;
use std::path::{Path, PathBuf};

macro_rules! assert_eq_logger {
    ($filename: expr, $logger: expr $(,)?) => {
        let correct_lines = read_lines_to_vec(format!("./test/vault/correct/{}", &$filename));
        assert_eq!($logger.stack.as_slice(), correct_lines);
    };
}

macro_rules! assert_eq_files {
    ($filename: expr $(,)?) => {
        let new_lines = read_lines_to_vec(format!("./test/vault/tmp/{}", &$filename));
        let corrected_lines = read_lines_to_vec(format!("./test/vault/correct/{}", &$filename));
        assert_eq!(new_lines, corrected_lines);
    };
}

#[test]
fn test_analyse_rows_0() {
    let filename = "2023-09-01.md";
    let mut test_logger = logger::TestLogger::default();

    let lines = read_lines_to_vec(format!("./test/vault/{}", filename));
    analyse_lines(&lines, &"To-Do".to_string(), &mut test_logger);
    analyse_lines_reverse(&lines);

    assert_eq_logger!(filename, test_logger);
}

#[test]
fn test_analyse_rows_1() {
    let filename = "2023-09-02.md";
    let mut test_logger = logger::TestLogger::default();

    let lines = read_lines_to_vec(format!("./test/vault/{}", filename));
    analyse_lines(&lines, &"To-Do".to_string(), &mut test_logger);
    analyse_lines_reverse(&lines);

    assert_eq_logger!(filename, test_logger);
}

#[test]
fn test_update_file() {
    let filename = "2023-09-03.md";
    let mut test_logger = logger::TestLogger::default();
    setup_test_file(format!("./test/vault/{}", filename)).ok();

    let lines = read_lines_to_vec(format!("./test/vault/tmp/{}", filename));
    let updated_lines = analyse_lines_reverse(&lines);

    write_lines(format!("./test/vault/tmp/{}", filename), &updated_lines).ok();

    assert_eq_files!(filename);
}

fn setup_test_file<P>(filename: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let source = PathBuf::from(filename.as_ref());
    let mut destination = source.clone();
    destination.pop();
    destination.push("tmp");
    destination.push(source.file_name().unwrap());

    fs::remove_dir_all("./test/vault/tmp").ok();
    fs::create_dir("./test/vault/tmp").ok();
    fs::copy(source, destination)?;
    Ok(())
}
