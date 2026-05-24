pub mod builtin;
pub mod command_parser;
pub mod shell;
use shell::Shell;

fn main() {
    let cwd_path_buf = std::env::current_dir().unwrap();
    let cwd = cwd_path_buf.to_str().unwrap().to_string();
    let mut shell = Shell::new(cwd);
    shell.init()
}
