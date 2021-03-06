//! GreeterScope implements two commands, one greets the user with the supplied name. The other
//! returns CommandResult::Quit to quit the application.

use cmdr::*;

/// Example scope that implements two commands, greet and quit
struct GreeterScope {}

#[cmdr(help = "Example scope", help_command = "hlp")]
impl GreeterScope {
    /// Cmdr command to greet someone.
    /// Takes one parameter and prints a greeting
    #[cmd(greet)]
    fn greet_method(&self, args: &[String]) -> CommandResult {
        println!("Hello {}", args[0]);
        Ok(Action::Done)
    }

    /// Cmdr command to quit the application by returning CommandResult::Quit
    #[cmd(quit, help = "Quit the application", alias(exit, x, q))]
    fn quit_method(&self, _args: &[String]) -> CommandResult {
        println!("Quitting");
        Ok(Action::Quit)
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() -> cmdr::Result<()> {
    cmd_loop(&mut GreeterScope {})?;
    Ok(())
}
