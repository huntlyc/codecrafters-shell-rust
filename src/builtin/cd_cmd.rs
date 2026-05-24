use crate::shell;
use std::{fs, path::PathBuf};

fn is_a_dir(path: &PathBuf) -> bool {
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

    let arg = cmd.args[0].to_string();

    // Go home
    if arg == "~" {
        let path = std::env::var("HOME").unwrap();
        shell.set_cwd(PathBuf::from(path));
        return;
    }

    // straight set to something like /usr/local/bin
    let new_path = PathBuf::from(arg);
    if new_path.is_absolute() && is_a_dir(&new_path) {
        shell.set_cwd(new_path);
        return;
    }

    // something like ../another-dir
    if new_path.is_relative() {
        let mut cwd = PathBuf::from(&shell.cwd);
        for part in new_path.iter() {
            let dir = String::from(part.to_str().unwrap());

            match dir.as_str() {
                ".." => {
                    cwd.pop();
                }
                "." => continue,
                _ => cwd.push(part),
            }
        }

        shell.set_cwd(cwd);

        return;
    }

    println!(
        "cd: {}: No such file or directory",
        String::from(new_path.to_str().unwrap())
    )
}
