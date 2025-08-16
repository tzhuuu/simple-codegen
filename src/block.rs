use std::fmt::{self, Write};

use crate::body::Body;
use crate::formatter::Formatter;

/// Defines a code block. This is used to define a function body.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Block {
    body: Vec<Body>,
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl Block {
    /// Creates an empty code block.
    pub fn new() -> Self {
        Block { body: Vec::new() }
    }

    /// Gets the body for the block.
    pub fn body(&self) -> &[Body] {
        &self.body
    }

    /// Sets the body for the block.
    pub fn set_body<B>(&mut self, body: impl IntoIterator<Item = B>) -> &mut Self
    where
        B: Into<Body>,
    {
        self.body = body.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the body for the block.
    pub fn with_body<B>(mut self, body: impl IntoIterator<Item = B>) -> Self
    where
        B: Into<Body>,
    {
        self.set_body(body);
        self
    }

    /// Returns a mutable reference to the body vector.
    pub fn body_mut(&mut self) -> &mut Vec<Body> {
        &mut self.body
    }

    /// Push a line to the code block.
    pub fn push_line(&mut self, line: impl Into<String>) -> &mut Self {
        self.body.push(Body::String(line.into()));
        self
    }

    /// Push a line to the code block.
    pub fn with_line(mut self, line: impl Into<String>) -> Self {
        self.push_line(line);
        self
    }

    /// Push a nested block to this block.
    pub fn push_block(&mut self, block: impl Into<Block>) -> &mut Self {
        self.body.push(Body::Block(block.into()));
        self
    }

    /// Push a nested block to this block.
    pub fn with_block(mut self, block: impl Into<Block>) -> Self {
        self.body.push(Body::Block(block.into()));
        self
    }

    /// Formats the block using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        // Inlined `Formatter::fmt`

        if !fmt.is_start_of_line() {
            write!(fmt, " ")?;
        }

        writeln!(fmt, "{{")?;

        fmt.indent(|fmt| {
            for b in &self.body {
                b.fmt(fmt)?;
            }

            Ok(())
        })?;

        write!(fmt, "}}")?;

        writeln!(fmt)?;
        Ok(())
    }
}
