use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines, Result, Write};
use std::path::Path;

pub fn read_lines_to_vec<P>(filename: P) -> Vec<String> where P: AsRef<Path> {
    let mut lines_vec: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            match line {
                Ok(line_content) => {
                    lines_vec.push(line_content);
                }
                Err(err) => {
                    eprintln!("Error reading line: {}", err);
                }
            }
        }
    }
    lines_vec
}

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);
    Ok(file_reader.lines())
}

pub fn write_lines<P>(filename: P, updated_lines: &Vec<String>) -> io::Result<()> where P: AsRef<Path> {
    let mut file = File::create(filename)?;
    for line in updated_lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}