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
    let path = "test.txt"; //since this is defined here it has no need to be mutable

    let mut flag = true;
    while flag {
        let lines = read_lines(path)?;
        for (i, line) in lines.iter().enumerate() {
            println!("{}: {}", i + 1, line);
        }
        println!("Choose an option: (1) Add line (2) Delete line (3) Exit ");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            
            "1" => {
                println!("Enter the line to add:");
                let mut new_line = String::new();
                io::stdin().read_line(&mut new_line)?;
                add_line(path, new_line.trim())?;
            }
            "2" => {
                println!("Enter the line number to delete:");
                let mut line_number = String::new();
                io::stdin().read_line(&mut line_number)?;
                if let Ok(num) = line_number.trim().parse::<usize>() {
                    delete_line(path, num)?;
                } else {
                    eprintln!("Please enter a valid number.");
                }
            }
            "3" => {
                flag = false;
            }
            _ => {
                eprintln!("Invalid option. Please try again.");
            }
        }
        
    }
    Ok(())
}