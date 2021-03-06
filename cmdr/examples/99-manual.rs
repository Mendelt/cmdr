//! Implementation of a cmdr application without using the #[cmdr] macro.
//! This example shows how to implement the Scope trait by hand. Normally you'd use the cmdr macro
//! to do the heavy lifting. This shows what the macro does under water.

use crate::line_writer::LineWriter;
use cmdr::*;

/// Example Cmdr scope
struct GreeterScope {}

impl GreeterScope {
    /// Cmdr command to greet someone.
    fn greet(&mut self, writer: &mut dyn LineWriter, args: &[String]) -> CommandResult {
        writer.write_line(format!("Hello {}", args[0]).as_ref());
        Ok(Action::Done)
    }

    /// Cmdr command to quit the application by returning CommandResult::Quit
    fn quit(&mut self) -> CommandResult {
        Ok(Action::Quit)
    }
}

/// Manual scope implementation for Cmdr. Normally you'd use the cmdr macro for this. Implements
/// the command method that dispatches commands to functions implemented above.
impl Scope for GreeterScope {
    fn commands(&self) -> ScopeDescription {
        ScopeDescription::new(
            Some("Manual greeter scope".to_string()),
            vec![
                ScopeCmdDescription::new("help".to_string(), Vec::new(), None),
                ScopeCmdDescription::new(
                    "greet".to_string(),
                    Vec::new(),
                    Some("Show a greeting.".to_string()),
                ),
                ScopeCmdDescription::new(
                    "quit".to_string(),
                    Vec::new(),
                    Some("Quit the application.".to_string()),
                ),
            ],
        )
    }

    fn run_command(
        &mut self,
        command: &ScopeCmdDescription,
        args: &[String],
        writer: &mut dyn LineWriter,
    ) -> CommandResult {
        match command.name() {
            "help" => self.help(args),
            "greet" => self.greet(writer, args),
            "quit" => self.quit(),
            _ => Err(Error::InvalidCommand(command.name().to_string())),
        }
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() -> cmdr::Result<()> {
    cmd_loop(&mut GreeterScope {})?;
    Ok(())
}
