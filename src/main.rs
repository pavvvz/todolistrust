use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn read_lines(path: &str) -> io::Result<Vec<String>> {
    //read_lines function takes a given path and returns a vector of strings representing the lines in the file
    let content = fs::read_to_string(path)?; //so this gives a string containing the entire content of the file, and thanks to the ? operator, it will return an error if the file cannot be read
    Ok(content.lines().map(|s| s.to_string()).collect()) //returning a vector of strings divided in lines, wrapped in an Ok result
}

fn add_line(path: &str, line: &str) -> io::Result<()> {
    //add_line function takes a given path and creates a new line, and returns a Result indicating success or failure
    let mut file = OpenOptions::new().append(true).create(true).open(path)?; //it either opens an existing file appending or creates a new one if it doesn't exist
    writeln!(file, "{}", line)?;
    Ok(())
}

fn delete_line(path: &str, line_number: usize) -> io::Result<()> {
    //delete_line function takes a given path and line number (which will be used as an index to delete the specified line), and returns a Result indicating success or failure
    let mut lines: Vec<String> = read_lines(path)?; //the result of read_lines is assigned to lines (which is a vector of strings)

    if line_number == 0 || line_number > lines.len() {
        //checks
        eprintln!("Invalid line number {}", line_number);
        return Ok(());
    }

    lines.remove(line_number - 1); // removes the line from the vector

    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;  //opens the file in write mode, deleting everything inside

    for l in lines {
        writeln!(file, "{}", l)?;
    } //writes every line

    Ok(())
}

fn main() -> io::Result<()> {
    let path = "test.txt";

    // Read
    println!("Initial file:");
    if let Ok(lines) = read_lines(path) {
        for (i, line) in lines.iter().enumerate() {
            println!("{}: {}", i + 1, line);
        }
    }

    // Add new line
    //add_line(path, "Hello, Rust!")?;
    //add_line(path, "Another row")?;

    // Delete row 1
    delete_line(path, 1)?;

    println!("\nAfter modifications:");
    if let Ok(lines) = read_lines(path) {
        for (i, line) in lines.iter().enumerate() {
            println!("{}: {}", i + 1, line);
        }
    }

    Ok(())
}
