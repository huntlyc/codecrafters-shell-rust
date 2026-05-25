use crate::shell;

/// Runs the command.
pub fn run(cmd: shell::Cmd, shell: &mut shell::Shell) {
    if cmd.args.len() == 0 {
        shell.cmd_not_found(&cmd.name);
        return;
    }

    if shell::is_builtin(&cmd.args[0].to_string()) {
        shell.std_out(&format!("{} is a shell builtin", &cmd.args[0]));
        return;
    }

    let exec_path = shell::get_exec_full_path(&cmd.args[0]);
    if exec_path != "" {
        shell.std_out(&format!("{} is {}", &cmd.args[0], exec_path));
        return;
    }

    shell.cmd_not_found(&cmd.args[0].to_string());
}
