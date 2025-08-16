use std::fmt::{self, Write};

use crate::block::Block;
use crate::formatter::Formatter;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Body {
    String(String),
    Block(Block),
}

impl Body {
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Body::String(s) => writeln!(fmt, "{}", s),
            Body::Block(b) => b.fmt(fmt),
        }
    }
}
