use crate::shell;

/// Runs the command.
pub fn run(cmd: shell::Cmd, shell: &mut shell::Shell) {
    shell.std_out(&format!("{}", cmd.args.join(" ")));
    shell.std_err(&"");
}
