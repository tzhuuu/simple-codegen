use std::fmt::{self, Write};

use crate::formatter::Formatter;

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct LineBreak {}

impl LineBreak {
    pub fn new() -> Self {
        Self {}
    }

    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        // We don't need to write a newline here because each `Item` gets written on a new line already.
        write!(fmt, "")?;
        Ok(())
    }
}
