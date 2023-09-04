use std::{env, io};
use crate::helpers::{read_lines_to_vec, write_lines};
use chrono::{Duration, NaiveDate};
use regex::Regex;

mod helpers;
mod logger;
#[cfg(test)]
mod test;

const TODO_SCHEMA_TO_COPY_AND_UPDATE: [&str; 1] = ["- [ ]"];
const TODO_SCHEMA_TO_COPY: [&str; 1] = ["- [/]"];
const TODO_SCHEMA_TO_CLEAR: [&str; 1] = ["- [>]"];
const SUBSECTION_SCHEMA: &str = "**";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let daily_notes_dir: &String = &args[1];
    let today_note_title: &String = &args[2];
    let section_title: String = match env::var("section_title") {
        Ok(val) => val,
        Err(_e) => "Today's Goals".to_string(),
    };

    // println!("{} {} {}", daily_notes_dir, today_note_title, section_title);

    let current_file_date = NaiveDate::parse_from_str(today_note_title, "%Y-%m-%d").unwrap();
    let previous_file_date = current_file_date - Duration::days(1);

    // println!("{} {}", current_file_date, previous_file_date);

    let file_path = format!(
        "{}/{}.md",
        daily_notes_dir,
        previous_file_date.format("%Y-%m-%d")
    );

    let lines = read_lines_to_vec(file_path.as_str());
    let updated_lines = analyse_lines(lines, &section_title, &mut logger::ConsoleLogger::default());

    write_lines(&file_path, &updated_lines)?;

    Ok(())
}

fn analyse_lines(
    lines: Vec<String>,
    section_title: &String,
    logger: &mut dyn logger::Logger,
) -> Vec<String> {
    let mut todo_section = false;
    let mut todo_subsection_title: Option<String> = None;
    let mut open_todos_per_subsection: Vec<String> = Vec::new();

    let mut updated_lines: Vec<String> = Vec::new();

    for line in lines {
        // updated_lines.push(line.clone());

        if !todo_section {
            if line.contains("# ") && line.contains(section_title) {
                todo_section = true;
                todo_subsection_title = None;
                open_todos_per_subsection.clear();
                updated_lines.push(line);
                continue;
            }
        } else {
            if line.contains("# ") {
                todo_section = false;
                updated_lines.push(line);
                continue;
            }

            if TODO_SCHEMA_TO_CLEAR
                .iter()
                .any(|schema| line.contains(schema))
            {
                let schema = Regex::new(r"- \[.]").unwrap();
                open_todos_per_subsection.push(schema.replace(line.as_str(), "- [ ]").to_string());
                updated_lines.push(line);
                continue;
            }

            if TODO_SCHEMA_TO_COPY
                .iter()
                .any(|schema| line.contains(schema))
            {
                open_todos_per_subsection.push(line.clone());
                updated_lines.push(line);
                continue;
            }

            if TODO_SCHEMA_TO_COPY_AND_UPDATE
                .iter()
                .any(|schema| line.contains(schema))
            {
                open_todos_per_subsection.push(line.clone());
                let schema = Regex::new(r"- \[.]").unwrap();
                updated_lines.push(schema.replace(line.as_str(), "- [>]").to_string());
                continue;
            }

            if line.contains(SUBSECTION_SCHEMA) {
                print_section(&todo_subsection_title, &open_todos_per_subsection, logger);
                todo_subsection_title = Some(String::from(line.clone()));
                open_todos_per_subsection.clear();
                updated_lines.push(line);
                continue;
            }
        }

        updated_lines.push(line);
    }

    print_section(&todo_subsection_title, &open_todos_per_subsection, logger);
    return updated_lines;
}

fn print_section(
    todo_subsection_title: &Option<String>,
    open_todos_per_subsection: &Vec<String>,
    logger: &mut dyn logger::Logger,
) {
    if !open_todos_per_subsection.is_empty() {
        if let Some(title) = todo_subsection_title {
            logger.log(format!("{}", title));
        }

        for todo in open_todos_per_subsection {
            logger.log(format!("{} ⭘", todo));
        }
        logger.log("".to_string());
    }
}
