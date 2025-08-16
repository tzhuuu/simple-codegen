use crate::generic_parameter::GenericParameter;
use crate::visibility::Vis;

/// Defines an [associated constant](https://doc.rust-lang.org/reference/items/associated-items.html#associated-constants).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AssociatedConst {
    name: String,
    ty: String,
    generics: Vec<GenericParameter>,
    concrete_vis: Vis,
    concrete_value: Option<String>,
}

impl AssociatedConst {
    /// Creates a new associated const.
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
            generics: Vec::new(),
            concrete_vis: Vis::Private,
            concrete_value: None,
        }
    }

    /// Gets the name of the associated const.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the associated const.
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }

    /// Sets the name of the associated const.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }

    /// Gets a mutable reference to the name of the associated const.
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Gets the type for this associated const.
    pub fn ty(&self) -> &str {
        &self.ty
    }

    /// Sets the type for this associated const.
    pub fn set_ty(&mut self, ty: impl Into<String>) -> &Self {
        self.ty = ty.into();
        self
    }

    /// Sets the type for this associated const.
    pub fn with_ty(&mut self, ty: impl Into<String>) -> &Self {
        self.set_ty(ty);
        self
    }

    /// Gets a mutable reference to the type of the associated const.
    pub fn ty_mut(&mut self) -> &mut String {
        &mut self.ty
    }

    /// Gets the generics for the associated const.
    pub fn generics(&self) -> &[GenericParameter] {
        &self.generics
    }

    /// Sets the generics for the associated const.
    pub fn set_generics<G>(&mut self, generics: impl IntoIterator<Item = G>) -> &mut Self
    where
        G: Into<GenericParameter>,
    {
        self.generics = generics.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the generics for the associated const.
    pub fn with_generics<G>(&mut self, generics: impl IntoIterator<Item = G>) -> &mut Self
    where
        G: Into<GenericParameter>,
    {
        self.set_generics(generics);
        self
    }

    /// Gets a mutable reference to the generics for the associated const.
    pub fn generics_mut(&mut self) -> &mut Vec<GenericParameter> {
        &mut self.generics
    }

    /// Gets the visibility of the concrete type.
    pub fn concrete_vis(&self) -> &Vis {
        &self.concrete_vis
    }

    /// Sets the visibility of the concrete type.
    pub fn set_concrete_vis(&mut self, vis: impl Into<Vis>) -> &mut Self {
        self.concrete_vis = vis.into();
        self
    }

    /// Sets the visibility of the concrete type.
    pub fn with_concrete_vis(mut self, vis: impl Into<Vis>) -> Self {
        self.set_concrete_vis(vis);
        self
    }

    /// Gets a mutable reference to the visibility of the concrete type.
    pub fn concrete_vis_mut(&mut self) -> &mut Vis {
        &mut self.concrete_vis
    }

    /// Gets the concrete value of the associated const, if any.
    pub fn concrete_value(&self) -> Option<&String> {
        self.concrete_value.as_ref()
    }

    /// Gets the concrete value for this associated const.
    pub fn set_concrete_value(&mut self, value: impl Into<String>) -> &mut Self {
        self.concrete_value = Some(value.into());
        self
    }

    /// Sets the concrete value for this associated const.
    pub fn with_concrete_value(mut self, value: impl Into<String>) -> Self {
        self.set_concrete_value(value);
        self
    }

    /// Gets a mutable reference to the concrete value of the associated const.
    pub fn concrete_value_mut(&mut self) -> Option<&mut String> {
        self.concrete_value.as_mut()
    }
}
