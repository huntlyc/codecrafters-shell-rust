use crate::shell;

/// runs the command
pub fn run(_: shell::Cmd, shell: &mut shell::Shell) {
    shell.std_out(&format!("{}", String::from(shell.cwd.to_str().unwrap())));
}
