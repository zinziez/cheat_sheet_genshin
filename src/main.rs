use std::fs::{File, ReadDir};
use std::io::{BufRead, BufReader};

fn read_char_file() {
    let filename = "src/character_list.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        println!("{}. {}", index + 1, line);
    }
}

fn main() {
    read_char_file()
}