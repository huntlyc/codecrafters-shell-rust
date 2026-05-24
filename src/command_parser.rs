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
                    //println!("STATE CHANGE: {:#?}", cur_state);
                } else if c == ' ' {
                    if cmd.name.len() == 0 {
                        cmd.name = buf.to_string();
                        //println!("Set CMD: {:#?}", cmd.name);
                    } else if buf.len() > 0 {
                        cmd.args.push(buf.to_string());
                        //println!("Set ARG: {:#?}", cmd.name);
                    }
                    buf = String::new();
                } else {
                    // Regular char
                    buf.push(c);
                    //println!("{} - {:#?}", c, cur_state);
                }
            }
            State::InSingleQuote => {
                if c == '\'' {
                    cur_state = State::CmdOrArg;
                    //println!("STATE CHANGE: {:#?}", cur_state);
                } else {
                    buf.push(c);
                    //println!("{} - {:#?}", c, cur_state);
                }
            }
        };
    }

    if buf.len() > 0 {
        cmd.args.push(buf.to_string());
    }

    //println!("{:#?}: {:#?}", cmd.name, cmd.args);

    return Ok(cmd);
}
