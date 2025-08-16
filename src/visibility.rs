use std::fmt;
use std::fmt::Write;

use crate::formatter::Formatter;

/// Enum representing the [visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html) of an item.
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub enum Vis {
    /// The default private visiblity
    #[default]
    Private,
    /// Equivalent of `pub`
    Pub,
    /// Equivalent of `pub(crate)`
    PubCrate,
    /// Equivalent of `pub(self)`
    PubSelf,
    /// Equivalent of `pub(super)`
    PubSuper,
    /// Custom visibility pub
    Custom(String),
}

impl Vis {
    /// Formats the enum using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Vis::Private => {}
            Vis::Pub => {
                write!(fmt, "pub ")?;
            }
            Vis::PubCrate => {
                write!(fmt, "pub(crate) ")?;
            }
            Vis::PubSelf => {
                write!(fmt, "pub(self) ")?;
            }
            Vis::PubSuper => {
                write!(fmt, "pub(super) ")?;
            }
            Vis::Custom(s) => {
                write!(fmt, "{} ", s)?;
            }
        }
        Ok(())
    }
}
