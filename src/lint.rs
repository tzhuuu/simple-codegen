use std::fmt::{self, Write};

use crate::formatter::Formatter;

/// Types of lint levels.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Lint {
    /// Corresponds to #[allow(...)]
    Allow(String),
    /// Corresponds to #[expect(...)]
    Expect(String),
    /// Corresponds to #[warn(...)]
    Warn(String),
    /// Corresponds to #[force-warn(...)]
    ForceWarn(String),
    /// Corresponds to #[deny(...)]
    Deny(String),
    /// Corresponds to #[forbid(...)]
    Forbid(String),
}

impl Lint {
    /// Corresponds to #[allow(...)]
    pub fn allow<S: Into<String>>(name: S) -> Self {
        Lint::Allow(name.into())
    }

    /// Corresponds to #[expect(...)]
    pub fn expect<S: Into<String>>(name: S) -> Self {
        Lint::Expect(name.into())
    }

    /// Corresponds to #[warn(...)]
    pub fn warn<S: Into<String>>(name: S) -> Self {
        Lint::Warn(name.into())
    }

    /// Corresponds to #[force-warn(...)]
    pub fn force_warn<S: Into<String>>(name: S) -> Self {
        Lint::ForceWarn(name.into())
    }

    /// Corresponds to #[deny(...)]
    pub fn deny<S: Into<String>>(name: S) -> Self {
        Lint::Deny(name.into())
    }

    /// Corresponds to #[forbid(...)]
    pub fn forbid<S: Into<String>>(name: S) -> Self {
        Lint::Forbid(name.into())
    }

    /// Format
    pub fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Lint::Allow(l) => {
                writeln!(fmt, "#[allow({})]", l)?;
            }
            Lint::Expect(l) => {
                writeln!(fmt, "#[expect({})]", l)?;
            }
            Lint::Warn(l) => {
                writeln!(fmt, "#[warn({})]", l)?;
            }
            Lint::ForceWarn(l) => {
                writeln!(fmt, "#[force-warn({})]", l)?;
            }
            Lint::Deny(l) => {
                writeln!(fmt, "#[deny({})]", l)?;
            }
            Lint::Forbid(l) => {
                writeln!(fmt, "#[forbid({})]", l)?;
            }
        }

        Ok(())
    }
}
