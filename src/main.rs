use std::fmt::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::{env, io};

use chrono::{Duration, NaiveDate};
use regex::Regex;

#[cfg(test)]
mod test;

mod logger;

const TODO_SCHEMA_TO_COPY: [&str; 2] = ["- [/]", "- [ ]"];
const TODO_SCHEMA_TO_CLEAR: [&str; 1] = ["- [>]"];
const SUBSECTION_SCHEMA: &str = "**";

fn main() -> Result<(), Error> {
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

    if let Ok(lines) = read_lines(file_path) {
        analyse_lines(lines, &section_title, &mut logger::ConsoleLogger::default());
    }

    Ok(())
}

fn analyse_lines(
    lines: Lines<BufReader<File>>,
    section_title: &String,
    logger: &mut dyn logger::Logger,
) {
    let mut todo_section = false;
    let mut todo_subsection_title: Option<String> = None;
    let mut open_todos_per_subsection: Vec<String> = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            if !todo_section {
                if line.contains("# ") && line.contains(section_title) {
                    todo_section = true;
                    todo_subsection_title = None;
                    open_todos_per_subsection.clear();
                    continue;
                }
            } else {
                if line.contains("# ") {
                    todo_section = false;
                    continue;
                }

                if line.contains(SUBSECTION_SCHEMA) {
                    print_section(&todo_subsection_title, &open_todos_per_subsection, logger);
                    todo_subsection_title = Some(String::from(line));
                    open_todos_per_subsection.clear();
                    continue;
                }

                if TODO_SCHEMA_TO_CLEAR.iter().any(|schema| line.contains(schema)) {
                    let schema = Regex::new(r"- \[.\]").unwrap();
                    open_todos_per_subsection.push(schema.replace(line.as_str(), "- [ ]").to_string());
                    continue;
                }

                if TODO_SCHEMA_TO_COPY.iter().any(|schema| line.contains(schema)) {
                    open_todos_per_subsection.push(line);
                    continue;
                }
            }
        }
    }
    print_section(&todo_subsection_title, &open_todos_per_subsection, logger);
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
