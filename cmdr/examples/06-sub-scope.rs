//! This example shows how to use CommandResult SubScope, Exit and Quit to open and close sub
//! scopes.
//! A sub scope is similar to a CommandResult::NewScope but switches back to the calling scope when
//! it finishes by returning CommandResult::Exit. CommandResult::Quit still quits the whole
//! application.

use cmdr::*;

struct MainScope {}

#[cmdr]
impl MainScope {
    fn prompt(&self) -> String {
        "main scope>".to_string()
    }

    #[cmd]
    /// Switch to the second scope
    fn sub(&mut self, _args: &[String]) -> CommandResult {
        CommandResult::sub_scope(SubScope {count: 1})
    }
}

/// The subscope. Has a quit and an exit command to quit the application or exit the sub scope and
/// return to the main scope, has a deeper_underground command to open another subscope.
struct SubScope {
    count: u32
}

#[cmdr]
impl SubScope {
    fn prompt(&self) -> String {
        format!("sub scope {} >", self.count)
    }

    #[cmd]
    fn exit(&mut self, _args: &[String]) -> CommandResult {
        CommandResult::Exit
    }

    #[cmd]
    fn quit(&mut self, _args: &[String]) -> CommandResult {
        CommandResult::Quit
    }

    #[cmd]
    fn deeper_undergroud(&mut self, _args: &[String]) -> CommandResult {
        CommandResult::sub_scope(SubScope {count: self.count + 1})
    }
}

fn main() {
    cmd_loop(&mut MainScope {});
}
