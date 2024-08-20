use apollo_parser::Error as ApolloError;
use std::{cmp::Ordering, error, fmt, iter, ops::ControlFlow};

#[derive(Clone, Debug)]
pub struct Error {
    pub(crate) errors: Vec<ApolloError>,
    pub(crate) input: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let line_bounds = iter::once(0)
            .chain(memchr::memchr_iter(b'\n', self.input.as_bytes()))
            .collect::<Vec<_>>();
        for (i, error) in self.errors.iter().enumerate() {
            let pos = error.index();
            let (ControlFlow::Break(line) | ControlFlow::Continue(line)) = line_bounds
                .iter()
                .try_fold(0, |i, offset| match pos.cmp(offset) {
                    Ordering::Less => ControlFlow::Break(i),
                    Ordering::Equal => ControlFlow::Continue(i),
                    Ordering::Greater => ControlFlow::Continue(i + 1),
                });
            let col = pos - line_bounds[line - 1];
            if i + 1 == self.errors.len() {
                write!(
                    f,
                    "syntax error at line {line}, col {col}: {}",
                    error.message()
                )?;
            } else {
                writeln!(
                    f,
                    "syntax error at line {line}, col {col}: {}",
                    error.message()
                )?;
            }
        }
        Ok(())
    }
}

impl error::Error for Error {}
