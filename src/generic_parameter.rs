use std::fmt::Write;

use crate::formatter::Formatter;

/// Defines a generic parameter.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct GenericParameter {
    name: String,
    traits: Vec<String>,
}

impl<S: Into<String>> From<S> for GenericParameter {
    fn from(value: S) -> Self {
        Self::new(value.into())
    }
}

impl GenericParameter {
    /// Creates a new generic parameter with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        GenericParameter {
            name: name.into(),
            traits: Vec::new(),
        }
    }

    /// Returns the name of the generic parameter.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the generic parameter.
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }

    /// Gets a mutable reference to the name of the generic parameter.
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Gets the traits for the generic parameter.
    pub fn traits(&self) -> &[String] {
        &self.traits
    }

    /// Sets the traits for the generic parameter.
    pub fn set_traits<T>(&mut self, traits: impl IntoIterator<Item = T>) -> &mut Self
    where
        T: Into<String>,
    {
        self.traits = traits.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the traits for the generic parameter.
    pub fn with_traits<T>(mut self, traits: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<String>,
    {
        self.set_traits(traits);
        self
    }

    /// Gets a mutable reference to the traits attached to the generic parameter.
    pub fn traits_mut(&mut self) -> &mut Vec<String> {
        &mut self.traits
    }

    /// Pushes a trait to the generic parameter.
    pub fn push_trait(&mut self, r#trait: impl Into<String>) -> &mut Self {
        self.traits.push(r#trait.into());
        self
    }

    /// Pushes a trait to the generic parameter.
    pub fn with_trait(mut self, r#trait: impl Into<String>) -> Self {
        self.push_trait(r#trait);
        self
    }

    /// Formats the generic parameter using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.name)?;
        if !self.traits.is_empty() {
            write!(fmt, ": ")?;
            for (i, t) in self.traits.iter().enumerate() {
                if i != 0 {
                    write!(fmt, " + ")?;
                }
                write!(fmt, "{}", t)?;
            }
        }
        Ok(())
    }
}
