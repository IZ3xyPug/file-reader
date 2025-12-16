use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    println!("My path is {:?}.", args);
    let file = File::open(args[0].clone());
    let file = match file {
        Ok(found) => found,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                },
                std::io::ErrorKind::PermissionDenied => {
                    panic!("Permission denied: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
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
