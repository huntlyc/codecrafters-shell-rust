use crate::shell;

/// runs the pwd command
pub fn run(shell: &mut shell::Shell) {
    println!("{}", shell.cwd);
}
