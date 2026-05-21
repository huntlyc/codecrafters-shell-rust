#[allow(unused_imports)]
use std::io::{self, Read, Write};

struct Command {
    name: String,
    args: Vec<String>,
}

fn main() {
    loop {
        print_prompt();
        let input = read_input();
        let cmd = parse_command_from_input(input);

        if !is_valid_command(&cmd.name) {
            println!("{}: command not found", cmd.name);
            continue;
        }

        run_cmd(cmd)
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

    return input;
}

fn parse_command_from_input(input: String) -> Command {
    let mut parts = input.split_whitespace();
    let cmd_name = parts.next().unwrap().to_string();
    let mut cmd_args: Vec<String> = Vec::new();

    for (_, a) in parts.enumerate() {
        cmd_args.push(a.to_string())
    }

    Command {
        name: cmd_name,
        args: cmd_args,
    }
}

fn is_valid_command(cmd: &String) -> bool {
    let cmds = ["echo", "exit"];

    return cmds.iter().any(|c| c == cmd);
}

fn run_cmd(cmd: Command) {
    match cmd.name.as_str() {
        "exit" => std::process::exit(0),
        "echo" => {
            println!("{}", cmd.args.join(" "))
        }
        _ => std::process::exit(1),
    }
}
