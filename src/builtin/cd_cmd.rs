use crate::shell;
use std::fs;

fn is_a_dir(path: &String) -> bool {
    let res = fs::metadata(path);

    match res {
        Ok(r) => r.is_dir(),
        _ => return false,
    }
}

/// Runs the 'type' command.
pub fn run(shell: &mut shell::Shell, cmd: shell::Cmd) {
    if cmd.args.len() == 0 {
        // @TODO: home
        return;
    }

    let new_path = cmd.args[0].to_string();
    if is_a_dir(&new_path) {
        shell.set_cwd(new_path);
        return;
    }
    println!("cd: {}: No such file or directory", new_path)
}
