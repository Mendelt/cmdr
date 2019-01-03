use std::io::stdout;
use std::io::stdin;
use std::io::Write;

pub use cmdr_derive::cmdr;

pub fn cmd_loop(context: &mut Scope) -> CommandResult {
    let mut last_result = CommandResult::Succes;

    while last_result == CommandResult::Succes {
        print!("{} ", context.prompt());
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        last_result = context.command(parse_line(&input));
    }

    last_result
}


pub enum Line<'a> {
    Empty,
    Command(&'a str, Vec<&'a str>)
}


#[derive(PartialEq)]
pub enum CommandResult {
    Succes,
    Quit
}


fn parse_line<'a>(line: &'a str) -> Line<'a> {
    let mut split_line = line.trim().split(" ");

    match split_line.next() {
        None => Line::Empty,
        Some(command) => Line::Command(command, split_line.collect()),
    }
}


pub trait Scope {
    fn prompt(&self) -> String {
        ">".to_string()
    }

    // Execute a single line, this function should eventually be generated by a macro
    fn command(&mut self, line: Line) -> CommandResult;

    // Execute an empty line, does nothing by default
    fn empty(&mut self) -> CommandResult { CommandResult::Succes }

    // Unknown command, shows an error by default
    fn default(&mut self, _line: Line) -> CommandResult {
        println!("Unknown command");
        CommandResult::Succes
    }
}