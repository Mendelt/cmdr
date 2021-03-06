use crate::{result::Error, Line};
use std::fmt::{Debug, Error as FmtError, Formatter};

/// Metadata describing a scope, is used to return help text and the list of commands that this
/// scope exposes.
#[derive(Debug)]
pub struct ScopeDescription {
    scope_help: Option<String>,
    methods: Vec<ScopeCmdDescription>,
}

impl ScopeDescription {
    /// Construct a command method list
    pub fn new(scope_help: Option<String>, methods: Vec<ScopeCmdDescription>) -> Self {
        ScopeDescription {
            scope_help,
            methods,
        }
    }

    /// Get all scope commands
    pub fn all_commands(&self) -> impl Iterator<Item = &ScopeCmdDescription> {
        self.methods.iter()
    }

    /// Find a command method by its command name or alias
    fn command_by_name(&self, name: &str) -> Option<&ScopeCmdDescription> {
        self.methods
            .iter()
            .filter(|method| method.handles(name))
            .next()
    }

    /// Return a command for an entered line
    pub fn command_for_line(&self, line: &Line) -> Option<&ScopeCmdDescription> {
        self.command_by_name(&line.command)
        // TODO: Check type and number of arguments
    }

    /// Format help text for command
    pub fn format_help_text(&self, command: Option<&str>) -> Result<String, Error> {
        if let Some(command) = command {
            Ok(self
                .command_by_name(command)
                .ok_or(Error::InvalidCommand(command.to_string()))?
                .help_text
                .clone()
                .ok_or(Error::NoHelpForCommand(command.to_string()))?)
        } else {
            Ok(self.format_scope_help())
        }
    }

    fn format_scope_help(&self) -> String {
        let mut result = String::new();

        result.push_str(match &self.scope_help {
            Some(scope_help) => scope_help,
            None => "These are the valid commands in this scope:",
        });

        result.push('\n');

        for command in self.all_commands() {
            result = format!("{}- {}\n", result, command.name());
        }

        result
    }
}

/// All information about a command method in one handy struct
pub struct ScopeCmdDescription {
    name: String,
    alias: Vec<String>,
    help_text: Option<String>,
}

impl ScopeCmdDescription {
    /// Construct a CmdMethod from a command name and a command closure
    pub fn new(name: String, alias: Vec<String>, help_text: Option<String>) -> Self {
        ScopeCmdDescription {
            name,
            alias,
            help_text,
        }
    }

    /// Name accessor method
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Help text accessor method
    pub fn help_text(&self) -> &Option<String> {
        &self.help_text
    }

    /// An iterator of all aliasses for this command
    pub fn aliases(&self) -> impl Iterator<Item = &String> {
        self.alias.iter()
    }

    /// Checks name or alias to see if a command can be handled.
    pub fn handles(&self, command: &str) -> bool {
        if self.name == command {
            true
        } else if self.alias.contains(&command.to_string()) {
            true
        } else {
            false
        }
    }
}

impl<'a> Debug for ScopeCmdDescription {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), FmtError> {
        formatter
            .debug_struct("ScopeCmdDescription")
            .field("name", &self.name)
            .field("alias", &self.alias)
            .field("help_text", &self.help_text)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_command() -> ScopeCmdDescription {
        ScopeCmdDescription::new(
            "test".to_string(),
            vec!["alias1".to_string(), "alias2".to_string()],
            Some("Help text\nMore lines".to_string()),
        )
    }

    #[test]
    fn command_should_not_handle_unknown() {
        let command = get_test_command();
        assert!(!command.handles("not_a_command"));
    }

    #[test]
    fn command_should_return_name() {
        let command = get_test_command();

        assert_eq!(command.name(), "test")
    }

    #[test]
    fn command_should_return_help_text() {
        let command = get_test_command();

        assert_eq!(
            command.help_text(),
            &Some("Help text\nMore lines".to_string())
        )
    }

    #[test]
    fn command_should_return_all_aliases() {
        let command = get_test_command();
        let aliasses: Vec<&String> = command.aliases().collect();

        assert_eq!(aliasses, vec!["alias1", "alias2"])
    }

    #[test]
    fn command_should_handle_by_name() {
        let command = get_test_command();

        assert!(command.handles("test"));
    }

    #[test]
    fn command_should_handle_by_alias() {
        let command = get_test_command();

        assert!(command.handles("alias1"));
        assert!(command.handles("alias2"));
    }
}
