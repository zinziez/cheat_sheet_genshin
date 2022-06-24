use colour::*;

pub fn read_user_input_exit() {
    println!("\nType 'exit' to exit the program.");
    println!("Otherwise, press enter to continue.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string().to_lowercase();
    if input == "exit" {
        std::process::exit(0);
    }
}

//TODO: Add a function that displays each characters respective information

pub fn venti() {
    println!("Ventis mat reqirements");
    println!("-------------------");
    green_ln!("5x Gold Bars");
    green_ln!("5x Iron Bars");
    green_ln!("5x Wood");
    green_ln!("5x Stone");
    green_ln!("5x Leather");
    read_user_input_exit();
}

pub fn sayu() {
    println!("Sayus mat reqirements");
    println!("-------------------");
    green_ln!("5x Gold Bars");
    green_ln!("5x Iron Bars");
    green_ln!("5x Wood");
    green_ln!("5x Stone");
    green_ln!("5x Leather");
    read_user_input_exit();
}
