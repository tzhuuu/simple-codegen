use crate::bound::Bound;

/// Defines an associated type.
///
/// https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AssociatedType {
    ty: Bound,
    concrete_ty: Option<(String, Vec<String>)>,
}

impl AssociatedType {
    /// Creates a new associated type with the provided name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ty: Bound::new(name, Vec::<String>::new()),
            concrete_ty: None,
        }
    }

    /// Creates a new associated type with the provided name and bounded traits.
    pub fn new_with_bounds(
        name: impl Into<String>,
        traits: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            ty: Bound::new(name, traits),
            concrete_ty: None,
        }
    }

    /// Gets the name of the associated type.
    pub fn name(&self) -> &str {
        self.ty.name()
    }

    /// Sets the name of the associated type.
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.ty.set_name(name);
        self
    }

    /// Sets the name of the associated type.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }

    /// Gets a mutable reference to the name of the associated type.
    pub fn name_mut(&mut self) -> &mut String {
        self.ty.name_mut()
    }

    /// Gets the associated type's bounds.
    pub fn trait_bounds(&self) -> &[String] {
        self.ty.traits()
    }

    /// Sets the associated type's bounds.
    pub fn set_trait_bounds<S>(&mut self, traits: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.ty.set_traits(traits);
        self
    }

    /// Sets the associated type's bounds.
    pub fn with_trait_bounds<S>(&mut self, traits: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.set_trait_bounds(traits);
        self
    }

    /// Gets a mutable reference to the associated type's bounds.
    pub fn trait_bounds_mut(&mut self) -> &mut Vec<String> {
        self.ty.traits_mut()
    }

    /// Pushes a trait bound to the associated type.
    pub fn push_trait_bound(&mut self, r#trait: impl Into<String>) -> &mut Self {
        self.ty.push_trait(r#trait.into());
        self
    }

    /// Pushes a trait bound to the associated type.
    pub fn with_trait_bound(mut self, r#trait: impl Into<String>) -> Self {
        self.push_trait_bound(r#trait);
        self
    }

    /// Gets the concrete type associated with this associated type, if any.
    pub fn concrete_ty(&self) -> Option<&(String, Vec<String>)> {
        self.concrete_ty.as_ref()
    }

    /// Sets the concrete type for this associated type.
    pub fn set_concrete_ty<S>(
        &mut self,
        name: impl Into<String>,
        generics: impl IntoIterator<Item = S>,
    ) -> &mut Self
    where
        S: Into<String>,
    {
        self.concrete_ty = Some((name.into(), generics.into_iter().map(Into::into).collect()));
        self
    }

    /// Sets the concrete type for this associated type.
    pub fn with_concrete_ty<S>(
        mut self,
        name: impl Into<String>,
        generics: impl IntoIterator<Item = S>,
    ) -> Self
    where
        S: Into<String>,
    {
        self.set_concrete_ty(name, generics);
        self
    }

    /// Gets a mutable reference to the concrete type for this associated type.
    pub fn concrete_ty_mut(&mut self) -> Option<&mut (String, Vec<String>)> {
        self.concrete_ty.as_mut()
    }
}
