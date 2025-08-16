use std::fmt::{self, Write};

use crate::formatter::Formatter;
use crate::generic_parameter::GenericParameter;

/// Defines a type.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Type {
    name: String,
    generics: Vec<GenericParameter>,
}

impl Type {
    /// Creates a new type with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Type {
            name: name.into(),
            generics: Vec::new(),
        }
    }

    /// Gets the name of the type.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the type.
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }

    /// Sets the name of the type.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }

    /// Gets a mutable reference to the name of the type.
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Returns the generics for the type.
    pub fn generics(&self) -> &[GenericParameter] {
        &self.generics
    }

    /// Sets the generics for the type.
    pub fn set_generics<G>(&mut self, generics: impl IntoIterator<Item = G>) -> &mut Self
    where
        G: Into<GenericParameter>,
    {
        self.generics = generics.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the generics for the type.
    pub fn with_generics<G>(mut self, generics: impl IntoIterator<Item = G>) -> Self
    where
        G: Into<GenericParameter>,
    {
        self.set_generics(generics);
        self
    }

    /// Gets a mutable reference to the generics attached to the type.
    pub fn generics_mut(&mut self) -> &mut Vec<GenericParameter> {
        &mut self.generics
    }

    /// Pushes a generic to the type.
    pub fn push_generic(&mut self, generic: impl Into<GenericParameter>) -> &mut Self {
        self.generics.push(generic.into());
        self
    }

    /// Pushes a generic to the type.
    pub fn with_generic(mut self, generic: impl Into<GenericParameter>) -> Self {
        self.push_generic(generic);
        self
    }

    /// Formats the type using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.name)?;
        Type::fmt_slice(&self.generics, fmt)
    }

    fn fmt_slice(generics: &[GenericParameter], fmt: &mut Formatter<'_>) -> fmt::Result {
        if !generics.is_empty() {
            write!(fmt, "<")?;

            for (i, g) in generics.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ", ")?
                }
                g.fmt(fmt)?;
            }

            write!(fmt, ">")?;
        }

        Ok(())
    }
}

impl<S: Into<String>> From<S> for Type {
    fn from(src: S) -> Self {
        Type {
            name: src.into(),
            generics: Vec::new(),
        }
    }
}
