//Hello and welcome to the genshin cheat sheet.
//This application was written by Xanthus58 and zinziez.
//This application is licensed under the MIT license.
//This application is open source and can be found on github at https://github.com/zinziez/cheat_sheet_genshin

//Allow modules
#![allow(clippy::needless_return)]

//Importing required libraries
use colour::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

//Importing required modules
mod char_materials;
use char_materials::*;

//A function that clears the screen
fn cls() {
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
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string().to_lowercase();
    return input;
}

//The main function where the core of the program is executed
fn main() {
    loop {
        let mut match_value = 0;
        cls();
        println!("Welcome to the Genshin Cheat Sheet!");
        println!("Please type the name of the character you want to learn about.");
        println!("Type 'exit' to exit the program.");
        println!("-------------------------------");
        let character = read_user_input();
        let charlist = read_file();
        //A for loop that checks if the user input matches any of the characters in the character_list.txt file
        if character == "exit" {
            std::process::exit(0);
        }
        for x in charlist {
            if x == character {
                cls();
                let x = x.as_str();
                match_value = 1;
                match x {
                    //TODO: match each character name to their function in the char_materials module
                    "venti" => venti(),
                    "sayu" => sayu(),
                    _ => println!("Character not found."),
                }
            }
        }
        if match_value == 0 {
            cls();
            red_ln!("Character not found.");
            red_ln!("Please try again.");
            read_user_input_exit();
        }
    }
}
