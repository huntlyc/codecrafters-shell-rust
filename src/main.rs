#[allow(unused_imports)]
use std::io::{self, Read, Write};

fn main() {
    loop {
        print_prompt();
        let cmd = read_input();

        if !is_valid_command(&cmd) {
            println!("{}: command not found", cmd);
        }

        run_cmd(cmd)
    }
}

fn run_cmd(cmd: String) {
    if cmd.to_lowercase() == "exit" {
        std::process::exit(0)
    }
}

fn is_valid_command(cmd: &String) -> bool {
    let cmds = ["exit"];

    return cmds.iter().any(|c| c == cmd);
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

    return input;
}
