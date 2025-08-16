use std::fmt::{self, Write};

use crate::formatter::Formatter;

/// Wrapper type over a documentation string.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Doc(String);

impl<S: Into<String>> From<S> for Doc {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

impl Doc {
    /// Create a new documentation string.
    pub fn new(doc: impl Into<String>) -> Self {
        Self(doc.into())
    }

    /// Gets the inner `String` type.
    pub fn as_inner(&self) -> &String {
        &self.0
    }

    /// Gets the mutable inner `String` type.
    pub fn as_inner_mut(&mut self) -> &mut String {
        &mut self.0
    }

    /// Formats the doc using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for line in self.0.lines() {
            write!(fmt, "///")?;
            if !line.is_empty() {
                write!(fmt, " {}", line)?;
            }
            writeln!(fmt)?;
        }

        Ok(())
    }
}
