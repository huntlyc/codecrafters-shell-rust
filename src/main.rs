#[allow(unused_imports)]
use std::io::{self, Read, Write};

fn main() {
    loop {
        print_prompt();
        read_input();
    }
}

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("should have provided input");

    input = input.trim().to_string();
    if input == "" {
        print_prompt();
        input = read_input()
    }

    println!("{}: command not found", input);

    return input;
}
