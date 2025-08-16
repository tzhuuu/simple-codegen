/// Defines a bound for a type in the `where` clause.
///
/// Note that [`GenericParameter`] also allows setting bounds right next to the generic parmaeters.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Bound {
    name: String,
    traits: Vec<String>,
}

impl Bound {
    /// Creates a new bound.
    pub fn new<S>(name: impl Into<String>, traits: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            traits: traits.into_iter().map(Into::into).collect(),
        }
    }

    /// Gets the name of the bound type.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the bound type.
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }

    /// Sets the name of the bound type.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }

    /// Gets a mutable reference to the name of the bound type.
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Gets the traits for the bound.
    pub fn traits(&self) -> &[String] {
        &self.traits
    }

    /// Sets the traits for the bound.
    pub fn set_traits<T>(&mut self, traits: impl IntoIterator<Item = T>) -> &mut Self
    where
        T: Into<String>,
    {
        self.traits = traits.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the traits on the bound.
    pub fn with_traits<T>(mut self, traits: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<String>,
    {
        self.set_traits(traits);
        self
    }

    /// Gets a mutable reference to the traits attached to the bound.
    pub fn traits_mut(&mut self) -> &mut Vec<String> {
        &mut self.traits
    }

    /// Pushes a trait to the bound.
    pub fn push_trait(&mut self, r#trait: impl Into<String>) -> &mut Self {
        self.traits.push(r#trait.into());
        self
    }

    /// Pushes a trait to the bound.
    pub fn with_trait(mut self, r#trait: impl Into<String>) -> Self {
        self.push_trait(r#trait);
        self
    }
}
