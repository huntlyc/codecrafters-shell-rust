fn main() {
    loop {
        shell::print_prompt();
        let input = shell::read_input();
        let cmd = shell::parse_command_from_input(input);

        shell::run_usr_cmd(cmd)
    }
}

#[allow(unused_imports)]
pub mod shell {
    use crate::builtin;
    use std::io::{self, Read, Write};
    use std::process::{Command, Stdio};
    use std::{fs, os::unix::fs::PermissionsExt};

    /// Represents a system command: "name", ["some", "args"]
    pub struct Cmd {
        pub name: String,
        pub args: Vec<String>,
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

    /// Matches cmd name against shell builtin commands
    pub fn is_builtin(cmd_namae: &String) -> bool {
        let builtins = ["echo", "exit", "pwd", "type"];
        builtins.iter().any(|e| e == cmd_namae)
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

    /// Tries to run the command that the user typed.
    pub fn run_usr_cmd(cmd: Cmd) {
        match cmd.name.as_str() {
            "exit" => std::process::exit(0),
            "echo" => {
                println!("{}", cmd.args.join(" "))
            }
            "type" => builtin::type_cmd::run(cmd),
            "pwd" => builtin::pwd_cmd::run(),
            _ => run(cmd),
        }
    }

    /// Prints out not found message.
    pub fn cmd_not_found(cmd_name: String) {
        println!("{}: not found", cmd_name);
    }
}

pub mod builtin {
    pub mod type_cmd {
        use crate::shell::{Cmd, cmd_not_found, get_exec_full_path, is_builtin};
        // Run the 'type' command.
        pub fn run(cmd: Cmd) {
            if cmd.args.len() == 0 {
                cmd_not_found(cmd.name);
                return;
            }

            if is_builtin(&cmd.args[0].to_string()) {
                println!("{} is a shell builtin", &cmd.args[0]);
                return;
            }

            let exec_path = get_exec_full_path(&cmd.args[0]);
            if exec_path != "" {
                println!("{} is {}", &cmd.args[0], exec_path);
                return;
            }

            cmd_not_found(cmd.args[0].to_string());
        }
    }

    pub mod pwd_cmd {
        pub fn run() {
            let path = std::env::current_dir().unwrap();
            println!("{}", path.display());
        }
    }
}
