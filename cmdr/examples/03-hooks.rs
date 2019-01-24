//! OverrideScope shows how default behavior like prompts, empty command and default command
//! handling can be overridden by implementing prompt, empty and default methods in our Scope

use cmdr::*;

struct ScopeWithHooks {}

/// Example scope that shows how to use the different hooks
#[cmdr]
impl ScopeWithHooks {
    fn before_loop(&mut self) {
        println!("This could be a good place to print a hello message for your user")
    }

    fn before_command(&mut self, _line: &Line) {
        println!("Code that gets executed before each command can go here.");
        println!("For now changing the line is not supported");
        // line = Line::Command(CommandLine {command, = "stuff" args: vec![]});
    }

    fn after_command(&mut self, _line: &Line, _result: CommandResult) -> CommandResult {
        println!("Code that gets executed after each command can go here.");
        println!("For example to change the command result to quit");

        CommandResult::Quit
    }

    fn after_loop(&mut self) {
        println!("This is the place where you can put code that gets run when the loop finishes");
        println!("Goodbye!");
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    let mut scope = ScopeWithHooks {};
    scope.cmd_loop();
}