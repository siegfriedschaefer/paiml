use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {

    // don't want to go deeper in this argument parsing, since rust has a lot of libraries for this.

    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);
    println!("Program name is {}.", args[1]);
    println!("Filename is {}.", args[2]);


    let file = File::open(args[2].clone());
    match file {
        Ok(file) => {
            println!("File opened successfully.");
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(error) => {
                        panic!("Error reading line: {}", error)
                    }
                }
            }
        }
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("File not found: {}", error)
                }
                _ => {
                    println!("Error opening file: {}", error)
                }
            }
        }
    };
    
}
