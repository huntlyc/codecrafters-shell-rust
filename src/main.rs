#[allow(unused_imports)]
use std::io::{self, Read, Write};
use std::{fs, os::unix::fs::PermissionsExt};

struct Command {
    name: String,
    args: Vec<String>,
}

fn main() {
    loop {
        print_prompt();
        let input = read_input();
        let cmd = parse_command_from_input(input);

        if !is_builtin(&cmd.name) {
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

fn is_builtin(cmd: &String) -> bool {
    let builtins = ["echo", "exit", "type"];
    builtins.iter().any(|e| e == cmd)
}

fn type_cmd(cmd: &String) -> bool {
    let path = std::env::var("PATH").unwrap();
    let mut dirs: Vec<String> = Vec::new();

    if path.contains(":") {
        let parts = path.split_terminator(":");
        for (_, d) in parts.enumerate() {
            dirs.push(d.to_string())
        }
    }

    for d in dirs {
        let full_path = d + "/" + cmd;
        let res = fs::metadata(&full_path);

        match res {
            Ok(r) => {
                if r.is_file() {
                    let mode = r.permissions().mode();
                    if mode & 0o111 != 0 {
                        println!("{} is {}", cmd, &full_path);
                        return true;
                    }
                } else {
                    continue;
                }
            }
            _ => continue,
        };
    }
    return false;
}

fn run_cmd(cmd: Command) {
    match cmd.name.as_str() {
        "exit" => std::process::exit(0),
        "echo" => {
            println!("{}", cmd.args.join(" "))
        }
        "type" => {
            if cmd.args.len() > 0 {
                if is_builtin(&cmd.args[0].to_string()) {
                    println!("{} is a shell builtin", &cmd.args[0])
                } else {
                    if !type_cmd(&cmd.args[0]) {
                        println!("{}: not found", &cmd.args[0])
                    }
                }
            } else {
                println!("{}: not found", &cmd.args[0])
            }
        }
        _ => std::process::exit(1),
    }
}
