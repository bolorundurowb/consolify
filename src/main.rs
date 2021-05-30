use std::io;
use std::collections::HashMap;

fn main() {
    let A = ["           ", "     /\\    ", "    /  \\   ", "   / /\\ \\  ", "  / ____ \\ ", " /_/__  \\_\\"];
    let B = ["  ____  ", " |  _ \\ ", " | |_) |", " |  _ < ", " | |_) |", " |____/_"];
    let C = ["   _____ ", "  / ____|", " | |     ", " | |     ", " | |     ", " \\_____|"];
    let D = ["  _____  ", " |  __ \\ ", " | |  | |", " | |  | |", " | |  | |", " |_____/"];
    let E = [" ______ ", " |  ____|", " | |__   ", " |  __|  ", " | |____ ", " |______|"];
    let F = ["  ______ ", " |  ____|", " | |__   ", " |  __|  ", " | |     ", " |_|     "];
    let G = ["_____","/ ____|","| |  __","| | |_ |","| |__| |","\\_____|"];
    let H = ["_    _","| |  | |","| |__| |","|  __  |","| |  | |","|_|  |_|"];
    let I = ["_____","|_   _|","| |","| |","_| |_","|_____|"];

    let mut character_map = HashMap::new();
    character_map.insert('A', A);
    character_map.insert('B', B);
    character_map.insert('C', C);
    character_map.insert('D', D);
    character_map.insert('E', E);
    character_map.insert('F', F);
    character_map.insert('G', G);
    character_map.insert('H', H);
    character_map.insert('I', I);

    println!("Please enter the text to display:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("An input must be provided.");

    let characters: Vec<char> = input.chars().collect();

    for character in characters {
        print_row(character_map[&character])
    }

    // println!("Your input was '{}'", input.trim());
}

fn print_row(row: [&str; 6]) {
    for character in &row {
        print!("{}", character);
    }
}