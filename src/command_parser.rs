use anyhow::{Result, anyhow};

use crate::shell::Cmd;

#[derive(Debug)]
enum State {
    CmdOrArg,
    InSingleQuote,
}

pub fn parse_command_from_input(input: String) -> Result<Cmd, anyhow::Error> {
    let mut cur_state = State::CmdOrArg;
    if input.len() == 0 {
        return Err(anyhow!("no input"));
    }

    let mut cmd = Cmd {
        name: String::from(""),
        args: Vec::new(),
    };

    let mut buf = String::new();
    for c in input.chars() {
        match cur_state {
            State::CmdOrArg => {
                if c == '\'' {
                    cur_state = State::InSingleQuote;
                } else if c == ' ' {
                    if cmd.name.len() == 0 {
                        cmd.name = buf.to_string();
                    } else {
                        cmd.args.push(buf.to_string());
                    }
                    buf = String::new();
                } else {
                    // Regular char
                    buf.push(c);
                }
            }
            State::InSingleQuote => {
                if c == '\'' {
                    cur_state = State::CmdOrArg;
                } else {
                    buf.push(c);
                }
            }
        };
    }

    if buf.len() > 0 {
        cmd.args.push(buf.to_string());
    }

    return Ok(cmd);
}
