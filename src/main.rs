#![allow(unused)]
use crate::helpers::{read_lines_to_vec, write_lines};
use chrono::{Duration, NaiveDate};
use regex::Regex;
use std::{env, io};

mod helpers;
mod logger;
#[cfg(test)]
mod test;

const TODO_SCHEMA_TO_COPY_AND_UPDATE: [&str; 1] = ["- [ ]"];
const TODO_SCHEMA_TO_COPY: [&str; 1] = ["- [/]"];
const TODO_SCHEMA_TO_CLEAR: [&str; 1] = ["- [>]"];
const SUBSECTION_SCHEMA: &str = "**";
// const SECTIONS_NOT_TO_REMOVE: [&str; 1] = ["End of Day"];
const SECTIONS_NOT_TO_REMOVE: [&str; 1] = ["End of Day"];

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
    analyse_lines(&lines, &section_title, &mut logger::ConsoleLogger::default());
    let updated_lines = analyse_lines_reverse(&lines);

    write_lines(&file_path, &updated_lines)?;

    Ok(())
}

fn analyse_lines(
    lines: &Vec<String>,
    todo_section_title: &String,
    logger: &mut dyn logger::Logger,
) -> Vec<String> {
    let mut todo_section = false;
    let mut todo_subsection_title: Option<String> = None;
    let mut open_todos_per_subsection: Vec<String> = Vec::new();

    let mut updated_lines: Vec<String> = Vec::new();

    for line in lines {

        if !todo_section {
            if line.contains("# ") && line.contains(todo_section_title) {
                todo_section = true;
                todo_subsection_title = None;
                open_todos_per_subsection.clear();
                updated_lines.push(line.clone());
                continue;
            }
        } else {
            if line.contains("# ") {
                todo_section = false;
                updated_lines.push(line.clone());
                continue;
            }

            if TODO_SCHEMA_TO_CLEAR
                .iter()
                .any(|schema| line.contains(schema))
            {
                let schema = Regex::new(r"- \[.]").unwrap();
                open_todos_per_subsection.push(schema.replace(line.as_str(), "- [ ]").to_string());
                updated_lines.push(line.clone());
                continue;
            }

            if TODO_SCHEMA_TO_COPY
                .iter()
                .any(|schema| line.contains(schema))
            {
                open_todos_per_subsection.push(line.clone());
                updated_lines.push(line.clone());
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
                updated_lines.push(line.clone());
                continue;
            }
        }

        updated_lines.push(line.clone());
    }

    print_section(&todo_subsection_title, &open_todos_per_subsection, logger);
    return updated_lines;
}

fn analyse_lines_reverse(lines: &Vec<String>) -> Vec<String> {

    let mut reverse_lines: Vec<String> = lines.clone();
    reverse_lines.reverse();
    let mut output_lines: Vec<String> = Vec::new();

    let mut section_lines: Vec<String> = Vec::new();
    let mut section_title: String;
    let headline_schema: Regex = Regex::new(r".*#{1,6} .*").unwrap();
    let empty_line_schema: Regex = Regex::new(r"^ +$").unwrap();

    for line in reverse_lines {
        if let Some(headline) = headline_schema.find(&line) {
            section_title = String::from(headline.as_str());
            if !SECTIONS_NOT_TO_REMOVE.iter().any(|not_to_remove_title| line.contains(not_to_remove_title)) {
                if section_lines.iter().all(|section_line| empty_line_schema.is_match(&section_line) || section_line.eq("")) {
                    section_lines.clear();
                    continue;
                }
            }

            output_lines.append(&mut section_lines.clone());
            section_lines.clear();
            output_lines.push(line.clone());
            continue;
        }

        if TODO_SCHEMA_TO_CLEAR
            .iter()
            .any(|schema| line.contains(schema))
        {
            let schema = Regex::new(r"- \[.]").unwrap();
            section_lines.push(line);
            continue;
        }

        if TODO_SCHEMA_TO_COPY
            .iter()
            .any(|schema| line.contains(schema))
        {
            section_lines.push(line);
            continue;
        }

        if TODO_SCHEMA_TO_COPY_AND_UPDATE
            .iter()
            .any(|schema| line.contains(schema))
        {
            let schema = Regex::new(r"- \[.]").unwrap();
            section_lines.push(schema.replace(line.as_str(), "- [>]").to_string());
            continue;
        }

        if line.contains(SUBSECTION_SCHEMA) {
            section_lines.push(line);
            continue;
        }

        section_lines.push(line.clone());

    }

    output_lines.reverse();
    output_lines.push(String::from(""));
    output_lines
}

fn push_stuff_to_print(mut stuff_to_print: Vec<String>, todo_section_title: &Option<String>, open_todos_per_subsection: &Vec<String>) -> Vec<String> {
    if !open_todos_per_subsection.is_empty() {
        if let Some(title) = todo_section_title {
            stuff_to_print.push(String::from(title));
        }
        for todo in open_todos_per_subsection {
            stuff_to_print.push(format!("{} ⭘", todo));
        }
    }
    stuff_to_print
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
