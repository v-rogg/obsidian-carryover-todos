use chrono::{Duration, NaiveDate};
use std::fmt::Error;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

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

    let mut todo_section = false;
    let mut todo_subsection_title: Option<String> = None;
    let mut open_todos_per_subsection: Vec<String> = Vec::new();

    let file_path = format!(
        "{}/{}.md",
        daily_notes_dir,
        previous_file_date.format("%Y-%m-%d")
    );

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                if !todo_section {
                    if line.contains("##") && line.contains(&section_title) {
                        todo_section = true;
                        continue;
                    }
                } else {
                    if line.contains("##") {
                        todo_section = false;
                        print_section(&todo_subsection_title, &open_todos_per_subsection);
                        continue;
                    }

                    if line.contains("**") {
                        print_section(&todo_subsection_title, &open_todos_per_subsection);
                        todo_subsection_title = Some(String::from(line));
                        open_todos_per_subsection.clear();
                        continue;
                    }

                    if line.contains("- [>]") {
                        open_todos_per_subsection.push(line.replace("- [>]", "- [ ]"));
                        continue;
                    }

                    if line.contains("- [/]") {
                        open_todos_per_subsection.push(line);
                        continue;
                    }

                    if line.contains("- [ ]") {
                        open_todos_per_subsection.push(line);
                        continue;
                    }
                }
            }
        }
    }

    Ok(())
}

fn print_section(todo_subsection_title: &Option<String>, open_todos_per_subsection: &Vec<String>) {
    if !open_todos_per_subsection.is_empty() {
        if let Some(title) = todo_subsection_title {
            println!("{}", title);
        }

        for todo in open_todos_per_subsection {
            println!("{} ⭘", todo);
        }
        println!();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
