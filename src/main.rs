pub mod builtin;
pub mod shell;

fn main() {
    loop {
        shell::print_prompt();
        let input = shell::read_input();
        let cmd = shell::parse_command_from_input(input);

        shell::run_usr_cmd(cmd)
    }
}
