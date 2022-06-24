//Importing required libraries
use std::fs::File;
use std::io::{BufRead, BufReader};

//A function that reads each line from character_list.txt and returns a vector of strings
fn read_file() -> Vec<String> {
    let file = File::open("src/character_list.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines
}

//Defining a function that can read user input
fn read_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .unwrap();
    let input = input.trim().to_string();
    return input;    
}

//The main function where the core of the program is executed
fn main() {
    let character = read_user_input();
    let charlist = read_file();
    for x in charlist {
        if x == character {
            println!("{} is a character", character);
        }
    }
}