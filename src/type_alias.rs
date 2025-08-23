use core::fmt;
use std::fmt::Write;

use crate::bound::Bound;
use crate::doc::Doc;
use crate::generic_parameter::GenericParameter;
use crate::lint::Lint;
use crate::type_def::TypeDef;
use crate::visibility::Vis;
use crate::{Formatter, Type};

/// Defines a [type alias](https://doc.rust-lang.org/reference/items/type-aliases.html).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeAlias {
    type_def: TypeDef,
    ty: Type,
}

impl TypeAlias {
    /// Creates a new type alias with the provided name and type.
    pub fn new(name: impl Into<String>, ty: impl Into<Type>) -> Self {
        Self {
            type_def: TypeDef::new(name.into()),
            ty: ty.into(),
        }
    }

    /// Gets the alias name.
    pub fn name(&self) -> &str {
        self.type_def.ty().name()
    }

    /// Sets the alias name.
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.type_def.ty_mut().set_name(name);
        self
    }

    /// Sets the alias name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }

    /// Gets a mutable reference to the alias name.
    pub fn name_mut(&mut self) -> &mut String {
        self.type_def.ty_mut().name_mut()
    }

    /// Gets the type definition.
    ///
    /// Interacting directly with the `TypeDef` is generally not preferred, but can be an escape hatch if there isn't an exposed method yet.
    pub fn type_def(&self) -> &TypeDef {
        &self.type_def
    }

    /// Sets the type definition.
    ///
    /// Interacting directly with the `TypeDef` is generally not preferred, but can be an escape hatch if there isn't an exposed method yet.
    pub fn set_type_def(&mut self, type_def: impl Into<TypeDef>) -> &mut Self {
        self.type_def = type_def.into();
        self
    }

    /// Sets the type definition.
    ///
    /// Interacting directly with the `TypeDef` is generally not preferred, but can be an escape hatch if there isn't an exposed method yet.
    pub fn with_type_def(mut self, type_def: impl Into<TypeDef>) -> Self {
        self.set_type_def(type_def);
        self
    }

    /// Gets a mutable reference to the type definition.
    ///
    /// Interacting directly with the `TypeDef` is generally not preferred, but can be an escape hatch if there isn't an exposed method yet.
    pub fn type_def_mut(&mut self) -> &mut TypeDef {
        &mut self.type_def
    }

    /// Gets the visibility.
    pub fn vis(&self) -> &Vis {
        self.type_def.vis()
    }

    /// Sets the visibility.
    pub fn set_vis(&mut self, vis: impl Into<Vis>) -> &mut Self {
        self.type_def.set_vis(vis.into());
        self
    }

    /// Sets the visibility.
    pub fn with_vis(mut self, vis: impl Into<Vis>) -> Self {
        self.set_vis(vis);
        self
    }

    /// Gets a mutable reference to the visibility.
    pub fn vis_mut(&mut self) -> &mut Vis {
        self.type_def.vis_mut()
    }

    /// Gets the generics.
    pub fn generics(&self) -> &[GenericParameter] {
        self.type_def.ty().generics()
    }

    /// Sets the generics.
    pub fn set_generics<G>(&mut self, generics: impl IntoIterator<Item = G>) -> &mut Self
    where
        G: Into<GenericParameter>,
    {
        self.type_def.ty_mut().set_generics(generics);
        self
    }

    /// Sets the generics.
    pub fn with_generics<G>(&mut self, generics: impl IntoIterator<Item = G>) -> &mut Self
    where
        G: Into<GenericParameter>,
    {
        self.set_generics(generics);
        self
    }

    /// Gets a mutable reference to the generics.
    pub fn generics_mut(&mut self) -> &mut Vec<GenericParameter> {
        self.type_def.ty_mut().generics_mut()
    }

    /// Pushes a generic.
    pub fn push_generic(&mut self, generic: impl Into<String>) -> &mut Self {
        self.type_def.ty_mut().push_generic(generic.into());
        self
    }

    /// Pushes a generic.
    pub fn with_generic(&mut self, generic: impl Into<String>) -> &mut Self {
        self.push_generic(generic);
        self
    }

    /// Gets the bounds.
    pub fn bounds(&self) -> &[Bound] {
        self.type_def.bounds()
    }

    /// Sets the bounds.
    pub fn set_bounds<B>(&mut self, bounds: impl IntoIterator<Item = B>) -> &mut Self
    where
        B: Into<Bound>,
    {
        self.type_def.set_bounds(bounds);
        self
    }

    /// Sets the bounds.
    pub fn with_bounds<B>(mut self, bounds: impl IntoIterator<Item = B>) -> Self
    where
        B: Into<Bound>,
    {
        self.set_bounds(bounds);
        self
    }

    /// Gets a mutable reference to the bounds.
    pub fn bounds_mut(&mut self) -> &mut Vec<Bound> {
        self.type_def.bounds_mut()
    }

    /// Pushes a `where` bound.
    pub fn push_bound(&mut self, bound: impl Into<Bound>) -> &mut Self {
        self.type_def.push_bound(bound.into());
        self
    }

    /// Pushes a `where` bound.
    pub fn with_bound(mut self, bound: impl Into<Bound>) -> Self {
        self.push_bound(bound);
        self
    }

    /// Gets the doc.
    pub fn doc(&self) -> Option<&Doc> {
        self.type_def.doc()
    }

    /// Sets the doc.
    pub fn set_doc<S>(&mut self, doc: impl Into<Option<S>>) -> &mut Self
    where
        S: Into<Doc>,
    {
        self.type_def.set_doc(doc);
        self
    }

    /// Sets the doc.
    pub fn with_doc<S>(mut self, doc: impl Into<Option<S>>) -> Self
    where
        S: Into<Doc>,
    {
        self.set_doc(doc);
        self
    }

    /// Gets a mutable reference to the doc.
    pub fn doc_mut(&mut self) -> Option<&mut Doc> {
        self.type_def.doc_mut()
    }

    /// Gets the derives.
    pub fn derives(&self) -> &[String] {
        self.type_def.derives()
    }

    /// Sets the derives.
    pub fn set_derives<S>(&mut self, derives: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.type_def.set_derives(derives);
        self
    }

    /// Sets the derives.
    pub fn with_derives<S>(mut self, derives: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_derives(derives);
        self
    }

    /// Gets a mutable reference to the derives.
    pub fn derives_mut(&mut self) -> &mut Vec<String> {
        self.type_def.derives_mut()
    }

    /// Pushes a new derive.
    pub fn push_derive(&mut self, derive: impl Into<String>) -> &mut Self {
        self.type_def.push_derive(derive.into());
        self
    }

    /// Pushes a new derive.
    pub fn with_derive(mut self, derive: impl Into<String>) -> Self {
        self.push_derive(derive);
        self
    }

    /// Gets the lint attributes.
    pub fn lints(&self) -> &[Lint] {
        self.type_def.lints()
    }

    /// Sets the lint attributes.
    pub fn set_lints<L>(&mut self, lints: impl IntoIterator<Item = L>) -> &mut Self
    where
        L: Into<Lint>,
    {
        self.type_def.set_lints(lints);
        self
    }

    /// Gets the lint attributes.
    pub fn with_lints<L>(mut self, lints: impl IntoIterator<Item = L>) -> Self
    where
        L: Into<Lint>,
    {
        self.set_lints(lints);
        self
    }

    /// Gets a mutable reference to the lint attributes.
    pub fn lints_mut(&mut self) -> &mut Vec<Lint> {
        self.type_def.lints_mut()
    }

    /// Pushes a lint attribute.
    pub fn push_lint(&mut self, lint: impl Into<Lint>) -> &mut Self {
        self.type_def.push_lint(lint.into());
        self
    }

    /// Pushes a lint attribute.
    pub fn with_lint(mut self, lint: impl Into<Lint>) -> Self {
        self.push_lint(lint);
        self
    }

    /// Gets the representation.
    pub fn repr(&self) -> Option<&String> {
        self.type_def.repr()
    }

    /// Sets the representation.
    pub fn set_repr<S>(&mut self, repr: impl Into<Option<S>>) -> &mut Self
    where
        S: Into<String>,
    {
        self.type_def.set_repr(repr);
        self
    }

    /// Sets the representation.
    pub fn with_repr<S>(mut self, repr: impl Into<Option<S>>) -> Self
    where
        S: Into<String>,
    {
        self.set_repr(repr);
        self
    }

    /// Gets the type alias's type.
    pub fn ty(&self) -> &Type {
        &self.ty
    }

    /// Sets the type alias's type.
    pub fn set_ty(&mut self, ty: impl Into<Type>) -> &mut Self {
        self.ty = ty.into();
        self
    }

    /// Sets the type alias's type.
    pub fn with_ty(mut self, ty: impl Into<Type>) -> Self {
        self.set_ty(ty);
        self
    }

    /// Gets a mutable reference to the type alias's type.
    pub fn ty_mut(&mut self) -> &mut Type {
        &mut self.ty
    }

    /// Formats the type alias using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.type_def.fmt_head("type", &[], fmt)?;
        write!(fmt, " = ")?;
        self.ty.fmt(fmt)?;
        write!(fmt, ";")?;
        Ok(())
    }
}
