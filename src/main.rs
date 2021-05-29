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

    let mut characterMap = HashMap::new();
    characterMap.insert('A', A);
    characterMap.insert('B', B);
    characterMap.insert('C', C);
    characterMap.insert('D', D);
    characterMap.insert('E', E);
    characterMap.insert('F', F);
    characterMap.insert('G', G);
    characterMap.insert('H', H);
    characterMap.insert('I', I);

    println!("Please enter the text to display:");
    // let mut input = String::new();

    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("An input must be provided.");

    // println!("Your input was '{}'", input.trim());
}

fn print_row(row: [String;10]) {
    for character in row {
        print!("{}", character);
    }
}