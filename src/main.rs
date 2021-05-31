use std::io;
use std::collections::HashMap;

fn main() {
    let A = ["           ", "     /\\    ", "    /  \\   ", "   / /\\ \\  ", "  / ____ \\ ", " /_/    \\_\\"];
    let B = ["  ____  ", " |  _ \\ ", " | |_) |", " |  _ < ", " | |_) |", " |____/ "];
    let C = ["   _____ ", "  / ____|", " | |     ", " | |     ", " | |     ", "  \\_____|"];
    let D = ["  _____  ", " |  __ \\ ", " | |  | |", " | |  | |", " | |  | |", " |_____/ "];
    let E = ["  ______ ", " |  ____|", " | |__   ", " |  __|  ", " | |____ ", " |______|"];
    let F = ["  ______ ", " |  ____|", " | |__   ", " |  __|  ", " | |     ", " |_|     "];
    let G = ["  _____ "," / ____| "," | |  __ "," | | |_ |"," | |__| |","  \\_____|"];
    let H = ["   _    _ "," | |  | |"," | |__| |"," |  __  |"," | |  | |"," |_|  |_|"];
    let I = ["  _____ "," |_   _|","   | |  ","   | |  ","  _| |_ "," |_____|"];

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

    for i in 0..6 {
        for character in input.chars() {
            if character_map.contains_key(&character) {
                print!("{}", character_map[&character][i]);
            }            
        }

        println!("");
    }

    

    // println!("Your input was '{}'", input.trim());
}