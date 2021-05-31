use std::io;
use std::collections::HashMap;

fn main() {
    let whitespace = ['\r', '\n', ' '];
    let A = ["           ", "     /\\    ", "    /  \\   ", "   / /\\ \\  ", "  / ____ \\ ", " /_/    \\_\\"];
    let B = ["  ____  ", " |  _ \\ ", " | |_) |", " |  _ < ", " | |_) |", " |____/ "];
    let C = ["   _____ ", "  / ____|", " | |     ", " | |     ", " | |     ", "  \\_____|"];
    let D = ["  _____  ", " |  __ \\ ", " | |  | |", " | |  | |", " | |  | |", " |_____/ "];
    let E = ["  ______ ", " |  ____|", " | |__   ", " |  __|  ", " | |____ ", " |______|"];
    let F = ["  ______ ", " |  ____|", " | |__   ", " |  __|  ", " | |     ", " |_|     "];
    let G = ["  _____ "," / ____| "," | |  __ "," | | |_ |"," | |__| |","  \\_____|"];
    let H = ["   _    _ "," | |  | |"," | |__| |"," |  __  |"," | |  | |"," |_|  |_|"];
    let I = ["  _____ "," |_   _|","   | |  ","   | |  ","  _| |_ "," |_____|"];
    let J = ["       _ ","      | |","      | |", "  _   | |", " | |__| |","  \\____/ "];
    let K = ["  _  __", " | |/ /", " | ' / ", " |  <  ", " | . \\ ", " |_|\\_\\"];
    let L = ["  _      ", " | |     ", " | |     ", " | |     ", " | |____ ", " |______|"];
    let M = ["  __  __ ", " |  \\/  |", " | \\  / |", " | |\\/| |", " | |  | |", " |_|  |_|"];
    let N = ["  _   _ ", " | \\ | |", " |  \\| |", " | . ` |", " | |\\  |", " |_| \\_|"];
    let O = ["   ____  ", "  / __ \\ ", " | |  | |", " | |  | |", " | |  | |", "  \\____/ "];
    let P = ["  _____  ", " |  __ \\ ", " | |__) |", " |  ___/ ", " | |     ", " |_|     "];
    let Q = ["   ____  ", "  / __ \\ ", " | |  | |", " | |  | |", " | |__| |", "  \\___\\_\\"];
    let R = ["  _____  ", " |  __ \\ ", " | |__) |", " |  _  / ", " | | \\ \\ ", " |_|  \\_\\"];
    let S = ["   _____ ", "  / ____|", " | (___  ", "  \\___ \\ ", "  ____) |", " |_____/ "];
    let T = ["  _______ ", " |__   __|", "    | |   ", "    | |   ", "    | |   ", "    |_|   "];
    let U = ["  _    _ ", " | |  | |", " | |  | |", " | |  | |", " | |__| |", "  \\____/ "];
    let V = [" __      __", " \\ \\    / /", "  \\ \\  / / ", "   \\ \\/ /  ", "    \\  /   ", "     \\/    "];
    let W = [" __          __", " \\ \\        / /", "  \\ \\  /\\  / / ", "   \\ \\/  \\/ /  ", "    \\  /\\  /   ", "     \\/  \\/    "];
    let X = [" __   __", " \\ \\ / /", "  \\ V / ", "   > <  ", "  / . \\ ", " /_/ \\_\\"];
    let Y = [" __     __", " \\ \\   / /", "  \\ \\_/ / ", "   \\   /  ", "    | |   ", "    |_|   "];
    let Z = ["  ______", " |___  /", "    / / ", "   / /  ", "  / /__ ", " /_____|"];
    let a = [];
    let b = [];
    let c = [];
    let d = [];
    let e = [];
    let f = [];
    let g = [];
    let h = [];
    let i = [];
    let j = [];
    let k = [];
    let l = [];
    let m = [];
    let n = [];
    let o = [];
    let p = [];
    let q = [];
    let r = [];
    let s = [];
    let t = [];
    let u = [];
    let v = [];
    let w = [];
    let x = [];
    let y = [];
    let z = [];
    let space = ["   ", "   ", "   ", "   ", "   ", "   "];

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
    character_map.insert('J', J);
    character_map.insert('J', K);
    character_map.insert('L', L);
    character_map.insert('M', M);
    character_map.insert('N', N);
    character_map.insert('O', O);
    character_map.insert('P', P);
    character_map.insert('Q', Q);
    character_map.insert('R', R);
    character_map.insert('S', S);
    character_map.insert('T', T);
    character_map.insert('U', U);
    character_map.insert('V', V);
    character_map.insert('W', W);
    character_map.insert('X', X);
    character_map.insert('Y', Y);
    character_map.insert('Z', Z);
    character_map.insert('a', a);
    character_map.insert('b', b);
    character_map.insert('c', c);
    character_map.insert('d', d);
    character_map.insert('e', e);
    character_map.insert('f', f);
    character_map.insert('g', g);
    character_map.insert('h', h);
    character_map.insert('i', i);
    character_map.insert('j', j);
    character_map.insert('k', k);
    character_map.insert('l', l);
    character_map.insert('m', m);
    character_map.insert('n', n);
    character_map.insert('o', o);
    character_map.insert('p', p);
    character_map.insert('q', q);
    character_map.insert('r', r);
    character_map.insert('s', s);
    character_map.insert('t', t);
    character_map.insert('u', u);
    character_map.insert('v', v);
    character_map.insert('w', w);
    character_map.insert('x', x);
    character_map.insert('y', y);
    character_map.insert('z', z);

    println!("Please enter the text to display:");
    let mut input = String::new();


    io::stdin()
        .read_line(&mut input)
        .expect("An input must be provided.");

    for i in 0..6 {
        for character in input.chars() {
            if character_map.contains_key(&character) {
                print!("{}", character_map[&character][i]);
            } else if whitespace.contains(&character) {
                print!("{}", space[i]);
            }            
        }

        println!("");
    }

    

    // println!("Your input was '{}'", input.trim());
}