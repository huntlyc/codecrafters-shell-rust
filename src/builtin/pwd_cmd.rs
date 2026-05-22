use crate::shell;

/// runs the pwd command
pub fn run(shell: &mut shell::Shell) {
    println!("{}", String::from(shell.cwd.to_str().unwrap()));
}
