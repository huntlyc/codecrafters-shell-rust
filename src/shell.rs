use crate::builtin;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf};

/// Represents a system command: "name", ["some", "args"]
pub struct Cmd {
    pub name: String,
    pub args: Vec<String>,
}

pub struct Shell {
    pub cwd: PathBuf,
}

impl Shell {
    pub fn new(cwd: String) -> Self {
        Shell {
            cwd: PathBuf::from(cwd),
        }
    }

    pub fn init(&mut self) {
        loop {
            print_prompt();
            let input = read_input();
            let cmd = parse_command_from_input(input);

            self.run_usr_cmd(cmd)
        }
    }

    /// Tries to run the command that the user typed.
    pub fn run_usr_cmd(&mut self, cmd: Cmd) {
        match cmd.name.as_str() {
            "exit" => std::process::exit(0),
            "echo" => {
                println!("{}", cmd.args.join(" "))
            }
            "type" => builtin::type_cmd::run(cmd),
            "pwd" => builtin::pwd_cmd::run(self),
            "cd" => builtin::cd_cmd::run(self, cmd),
            _ => run(cmd),
        }
    }

    pub fn set_cwd(&mut self, cwd: PathBuf) {
        self.cwd = cwd;
    }
}

/// The base user prompt
pub fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

/// Reads the input from the user.
pub fn read_input() -> String {
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

/// Given: echo hello world
/// Returns: Cmd { name: "echo", args: ["hello", "world"]
pub fn parse_command_from_input(input: String) -> Cmd {
    let mut parts = input.split_whitespace();
    let mut cmd_args: Vec<String> = Vec::new();

    let cmd_name = parts.next().unwrap().to_string();

    for (_, a) in parts.enumerate() {
        cmd_args.push(a.to_string())
    }

    Cmd {
        name: cmd_name,
        args: cmd_args,
    }
}

/// Runs a cmd
fn run(cmd: Cmd) {
    let cmd_with_path = get_exec_full_path(&cmd.name);

    if cmd_with_path == "" {
        println!("{}: not found", &cmd.name);
        return;
    }
    Command::new(&cmd.name)
        .stdout(Stdio::inherit())
        .args(&cmd.args)
        .output()
        .unwrap();
}

/// Given a command name, search the PATH for the command.
/// Returns the full path, e.g. /usr/bin/ls
pub fn get_exec_full_path(cmd_name: &String) -> String {
    let path = std::env::var("PATH").unwrap();
    let mut dirs: Vec<String> = Vec::new();

    if path.contains(":") {
        let parts = path.split_terminator(":");
        for (_, d) in parts.enumerate() {
            dirs.push(d.to_string())
        }
    }

    for d in dirs {
        let full_path = d + "/" + cmd_name;
        let res = fs::metadata(&full_path);

        match res {
            Ok(r) => {
                if r.is_file() {
                    let mode = r.permissions().mode();
                    if mode & 0o111 != 0 {
                        return full_path;
                    }
                }
            }
            _ => continue,
        };
    }
    return "".to_string();
}

/// Matches cmd name against shell builtin commands
pub fn is_builtin(cmd_namae: &String) -> bool {
    let builtins = ["echo", "exit", "pwd", "type", "cd"];
    builtins.iter().any(|e| e == cmd_namae)
}

/// Prints out not found message.
pub fn cmd_not_found(cmd_name: String) {
    println!("{}: not found", cmd_name);
}
