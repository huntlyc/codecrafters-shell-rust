use crate::shell;

/// Runs the 'type' command.
pub fn run(cmd: shell::Cmd) {
    if cmd.args.len() == 0 {
        shell::cmd_not_found(cmd.name);
        return;
    }

    if shell::is_builtin(&cmd.args[0].to_string()) {
        println!("{} is a shell builtin", &cmd.args[0]);
        return;
    }

    let exec_path = shell::get_exec_full_path(&cmd.args[0]);
    if exec_path != "" {
        println!("{} is {}", &cmd.args[0], exec_path);
        return;
    }

    shell::cmd_not_found(cmd.args[0].to_string());
}
