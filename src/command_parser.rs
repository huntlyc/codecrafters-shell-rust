use anyhow::{Result, anyhow};

use crate::shell::{Cmd, Shell};

#[derive(Debug)]
enum State {
    CmdOrArg,
    InSingleQuote,
    InDoubleQuote,
    EscapeNonQuoteChar,
    EscapeDoubleQuoteChar,
    RedirectingStdOut,
    //RedirectingStdErr,
}

pub fn parse_command_from_input(input: String, shell: &mut Shell) -> Result<Cmd, anyhow::Error> {
    let debug = false;
    let mut cur_state = State::CmdOrArg;
    if input.len() == 0 {
        return Err(anyhow!("no input"));
    }

    let mut cmd = Cmd {
        name: String::from(""),
        args: Vec::new(),
    };

    let mut buf = String::new();
    let mut output_file_buf = String::new();
    let mut full_input_buf = String::new();
    //let mut output_file_buf = String::new();

    for c in input.chars() {
        full_input_buf.push(c);
        match cur_state {
            State::CmdOrArg => {
                if c == '\\' {
                    cur_state = State::EscapeNonQuoteChar;
                } else if c == '\'' {
                    cur_state = State::InSingleQuote;
                    if debug {
                        println!("STATE CHANGE: {:#?}", cur_state);
                    }
                } else if c == '\"' {
                    cur_state = State::InDoubleQuote;
                    if debug {
                        println!("STATE CHANGE: {:#?}", cur_state);
                    }
                } else if c == '>' {
                    cur_state = match full_input_buf.chars().last().unwrap() {
                        ' ' => State::RedirectingStdOut,
                        '1' => State::RedirectingStdOut,
                        _ => State::RedirectingStdOut,
                    };

                    buf = String::new(); // possibly has 1/2 in it - discard

                    if debug {
                        println!("STATE CHANGE: {:#?}", cur_state);
                    }
                } else if c == ' ' {
                    if cmd.name.len() == 0 {
                        cmd.name = buf.to_string();
                        if debug {
                            println!("Set CMD: {:#?}", cmd.name);
                        }
                    } else if buf.len() > 0 {
                        cmd.args.push(buf.to_string());
                        if debug {
                            println!("Set ARG: {:#?}", cmd.name);
                        }
                    }
                    buf = String::new();
                } else {
                    // Regular char
                    buf.push(c);
                    if debug {
                        println!("{} - {:#?}", c, cur_state);
                    }
                }
            }
            State::EscapeNonQuoteChar => {
                buf.push(c);
                cur_state = State::CmdOrArg;
            }
            State::EscapeDoubleQuoteChar => {
                buf.push(c);
                cur_state = State::InDoubleQuote;
            }
            State::InSingleQuote => {
                if c == '\'' {
                    cur_state = State::CmdOrArg;
                    if debug {
                        println!("STATE CHANGE: {:#?}", cur_state);
                    }
                } else {
                    buf.push(c);
                    if debug {
                        println!("{} - {:#?}", c, cur_state);
                    }
                }
            }
            State::InDoubleQuote => {
                if c == '\"' {
                    cur_state = State::CmdOrArg;
                    if debug {
                        println!("STATE CHANGE: {:#?}", cur_state);
                    }
                } else if c == '\\' {
                    cur_state = State::EscapeDoubleQuoteChar;
                } else {
                    buf.push(c);
                    if debug {
                        println!("{} - {:#?}", c, cur_state);
                    }
                }
            }
            State::RedirectingStdOut => {
                output_file_buf.push(c);
            }
        };
    }

    if buf.len() > 0 {
        if cmd.name.len() == 0 {
            cmd.name = buf.to_string();
        } else {
            cmd.args.push(buf.to_string());
        }
    }

    if output_file_buf.len() > 0 {
        shell.set_std_out(&output_file_buf.trim())
    }

    if debug {
        println!("{:#?}: {:#?}", cmd.name, cmd.args);
    }
    if debug {
        println!("{:#?}", shell);
    }

    return Ok(cmd);
}
