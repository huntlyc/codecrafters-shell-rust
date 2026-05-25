use crate::builtin;
use crate::command_parser;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf};

/// Represents a system command: "name", ["some", "args"]
pub struct Cmd {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub enum Output {
    Print,
    Redirect,
}

#[derive(Debug)]
pub struct Shell {
    pub cwd: PathBuf,
    pub std_out: Output,
    pub std_out_file: String,
    pub std_err: Output,
    pub std_err_file: String,
}

impl Shell {
    pub fn new(cwd: String) -> Self {
        Shell {
            cwd: PathBuf::from(cwd),
            std_out: Output::Print,
            std_out_file: String::new(),
            std_err: Output::Print,
            std_err_file: String::new(),
        }
    }

    pub fn init(&mut self) {
        loop {
            print_prompt();
            let input = read_input();
            let cmd = command_parser::parse_command_from_input(input, self).unwrap();

            self.run_usr_cmd(cmd)
        }
    }

    pub fn set_std_out(&mut self, fname: &str) {
        self.std_out = Output::Redirect;
        self.std_out_file = String::from(fname);
    }

    pub fn set_std_err(&mut self, fname: &str) {
        self.std_err = Output::Redirect;
        self.std_err_file = String::from(fname);
    }

    pub fn std_out(&mut self, str: &str) {
        match self.std_out {
            Output::Print => println!("{}", str),
            Output::Redirect => self.write_to_file(&str),
        }
    }

    fn write_to_file(&mut self, contents: &str) {
        match fs::write(&self.std_out_file, contents) {
            Err(e) => self.std_err(&format!("{}", e.to_string())),
            _ => return,
        }
    }

    pub fn std_err(&mut self, str: &str) {
        eprintln!("{}", str);
    }

    /// Tries to run the command that the user typed.
    pub fn run_usr_cmd(&mut self, cmd: Cmd) {
        match cmd.name.as_str() {
            "exit" => std::process::exit(0),
            "echo" => builtin::echo_cmd::run(cmd, self),
            "type" => builtin::type_cmd::run(cmd, self),
            "pwd" => builtin::pwd_cmd::run(cmd, self),
            "cd" => builtin::cd_cmd::run(cmd, self),
            _ => run(cmd, self),
        }
    }

    pub fn set_cwd(&mut self, cwd: PathBuf) {
        self.cwd = cwd;
        match std::env::set_current_dir(self.cwd.clone()) {
            Err(e) => self.std_err(&format!("{}", e.to_string())),
            _ => return,
        }
    }
    /// Prints out not found message.
    pub fn cmd_not_found(&mut self, cmd_name: &str) {
        self.std_err(&format!("{}: not found", cmd_name));
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

/// Runs a cmd
fn run(cmd: Cmd, shell: &mut Shell) {
    let cmd_with_path = get_exec_full_path(&cmd.name);

    if cmd_with_path == "" {
        shell.cmd_not_found(&cmd.name);
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
pub fn get_exec_full_path(cmd_name: &str) -> String {
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
pub fn is_builtin(cmd_name: &str) -> bool {
    let builtins = ["echo", "exit", "pwd", "type", "cd"];
    builtins.iter().any(|e| *e == cmd_name)
}
