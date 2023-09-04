use super::*;

#[test]
fn test_analyse_rows_0() {
    let section_title = "To-Do's".to_string();
    let mut test_logger = logger::TestLogger::default();
    if let Ok(lines) = read_lines("./test/vault/2023-09-01.md") {
        analyse_lines(lines, &section_title, &mut test_logger);
    }
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
    let section_title = "To-Do's".to_string();
    let mut test_logger = logger::TestLogger::default();
    if let Ok(lines) = read_lines("./test/vault/2023-09-02.md") {
        analyse_lines(lines, &section_title, &mut test_logger);
    }
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
