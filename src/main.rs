//Importing required libraries
use std::fs::File;
use std::io::{BufRead, BufReader};
mod char_materials;
use char_materials::*;

//A function that clears the screen
fn cls(){
    print!("{esc}c", esc = 27 as char); 
}

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
    let input = input.trim().to_string().to_lowercase();
    return input;    
}

//function that reads user input before exiting the program
fn read_user_input_exit() -> String {
    println!("\nPress enter to exit");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .unwrap();
    let input = input.trim().to_string().to_lowercase();
    if input == "exit" {
        std::process::exit(0);
    }
    return input;    
}

//The main function where the core of the program is executed
fn main() {
    cls();
    println!("Welcome to the Genshin Cheat Sheet!");
    println!("Please type the name of the character you want to learn about.");
    println!("-------------------------------");
    let character = read_user_input();
    let charlist = read_file();
    for x in charlist {
        if x == character {
            cls();
            if character == "venti" {
                venti();
            } else if character == "sayu" {
                sayu();
            }
            read_user_input_exit();
        }
    }
}