//Importing required libraries
use std::fs::File;
use std::io::{BufRead, BufReader};

//Defining a function to read the characters from the file
fn read_char_file() {
    let filename = "src/character_list.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename)
        .unwrap();
    let reader = BufReader::new(file);
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader
        .lines()
        .enumerate() {
        let line = line
            .unwrap(); // Ignore errors.
        // Show the line and its number.
        println!("{}", line);
    }
}

//Defining a function that can read user input
fn read_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .unwrap();
    return input;    
}

//The main function where the core of the program is executed
fn main() {
    let character = read_user_input();
    read_char_file();
}