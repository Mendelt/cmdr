use crate::line_reader::LineReader;
use crate::scope::Scope;
use crate::CommandResult;
use std::fmt::Debug;

/// Wraps a LineReader and a Scope and allows using the scope to interpret commands from the
/// LineReader
#[derive(Debug)]
pub struct Runner<S: Scope, R: LineReader> {
    reader: Box<R>,
    scope: S,
}

impl<S: Scope, R: LineReader> Runner<S, R> {
    /// Create a new runner that takes lines from the `reader` and executes them using the `scope`
    pub fn new(reader: R, scope: S) -> Self {
        Runner { reader: Box::new(reader), scope }
    }

    /// Start reading lines and executing them
    pub fn run(&mut self) -> CommandResult {
        let mut result = self.scope.run_lines(self.reader.as_mut());

        while let CommandResult::NewScope(scope_runner) = result {
            result = scope_runner.run_lines(self.reader.as_mut());
        }

        result
    }
}
