use std::fmt;

use crate::bound::Bound;
use crate::doc::Doc;
use crate::formatter::Formatter;
use crate::generic_parameter::GenericParameter;
use crate::lint::Lint;
use crate::r#type::Type;
use crate::type_def::TypeDef;
use crate::variant::Variant;
use crate::visibility::Vis;

/// Defines an [enum](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Enum {
    type_def: TypeDef,
    variants: Vec<Variant>,
}

impl Enum {
    /// Creates an enum definition with the provided name.
    pub fn new(name: impl Into<String>) -> Self {
        Enum {
            type_def: TypeDef::new(name.into()),
            variants: Vec::new(),
        }
    }

    /// Gets the name of the enum.
    pub fn name(&self) -> &str {
        self.type_def.ty().name()
    }

    /// Sets the name of the enum.
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.type_def.ty_mut().set_name(name.into());
        self
    }

    /// Sets the name of the enum.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }

    /// Gets a mutable reference to the name of the enum.
    pub fn name_mut(&mut self) -> &mut String {
        self.type_def.ty_mut().name_mut()
    }

    /// Returns a reference to the type.
    pub fn ty(&self) -> &Type {
        self.type_def.ty()
    }

    /// Sets the type.
    pub fn set_ty(&mut self, ty: impl Into<Type>) -> &mut Self {
        self.type_def.set_ty(ty.into());
        self
    }

    /// Sets the type.
    pub fn with_ty(mut self, ty: impl Into<Type>) -> Self {
        self.set_ty(ty);
        self
    }

    /// Gets a mutable reference to the type.
    pub fn ty_mut(&mut self) -> &mut Type {
        self.type_def.ty_mut()
    }

    /// Gets a reference to the visibility.
    pub fn vis(&self) -> &Vis {
        self.type_def.vis()
    }

    /// Sets the enum visibility.
    pub fn set_vis(&mut self, vis: impl Into<Vis>) -> &mut Self {
        self.type_def.set_vis(vis.into());
        self
    }

    /// Set sthe enum visibility.
    pub fn with_vis(mut self, vis: impl Into<Vis>) -> Self {
        self.set_vis(vis);
        self
    }

    /// Gets a mutable reference to the enum visibility.
    pub fn vis_mut(&mut self) -> &mut Vis {
        self.type_def.vis_mut()
    }

    /// Sets the generics for this enum.
    pub fn generics(&self) -> &[GenericParameter] {
        self.type_def.ty().generics()
    }

    /// Sets the generics for this enum.
    pub fn set_generics<G>(&mut self, generics: impl IntoIterator<Item = G>) -> &mut Self
    where
        G: Into<GenericParameter>,
    {
        self.type_def.ty_mut().set_generics(generics);
        self
    }

    /// Sets the generics for this enum.
    pub fn with_generics<G>(&mut self, generics: impl IntoIterator<Item = G>) -> &mut Self
    where
        G: Into<GenericParameter>,
    {
        self.set_generics(generics);
        self
    }

    /// Gets a mutable reference to the generics for this enum.
    pub fn generics_mut(&mut self) -> &mut Vec<GenericParameter> {
        self.type_def.ty_mut().generics_mut()
    }

    /// Pushes a generic to the enum.
    pub fn push_generic(&mut self, generic: impl Into<String>) -> &mut Self {
        self.type_def.ty_mut().push_generic(generic.into());
        self
    }

    /// Pushes a generic to the enum.
    pub fn with_generic(mut self, generic: impl Into<String>) -> Self {
        self.push_generic(generic);
        self
    }

    /// Sets the bounds for this enum.
    pub fn bounds(&self) -> &[Bound] {
        self.type_def.bounds()
    }

    /// Sets the bounds for this enum.
    pub fn set_bounds<B>(&mut self, bounds: impl IntoIterator<Item = B>) -> &mut Self
    where
        B: Into<Bound>,
    {
        self.type_def.set_bounds(bounds);
        self
    }

    /// Sets the bounds for this enum.
    pub fn with_bounds<B>(mut self, bounds: impl IntoIterator<Item = B>) -> Self
    where
        B: Into<Bound>,
    {
        self.set_bounds(bounds);
        self
    }

    /// Gets a mutable reference to the bounds for this enum.
    pub fn bounds_mut(&mut self) -> &mut Vec<Bound> {
        self.type_def.bounds_mut()
    }

    /// Pushes a `where` bound to the enum.
    pub fn push_bound(&mut self, bound: impl Into<Bound>) -> &mut Self {
        self.type_def.push_bound(bound.into());
        self
    }

    /// Pushes a `where` bound to the enum.
    pub fn with_bound(&mut self, bound: impl Into<Bound>) -> &mut Self {
        self.push_bound(bound);
        self
    }

    /// Gets the enum documentation.
    pub fn doc(&self) -> Option<&Doc> {
        self.type_def.doc()
    }

    /// Sets the enum documentation.
    pub fn set_doc<S>(&mut self, doc: impl Into<Option<S>>) -> &mut Self
    where
        S: Into<Doc>,
    {
        self.type_def.set_doc(doc);
        self
    }

    /// Sets the enum documentation.
    pub fn with_doc(mut self, doc: impl Into<Doc>) -> Self {
        self.set_doc(doc);
        self
    }

    /// Gets a mutable reference to the enum documentation.
    pub fn doc_mut(&mut self) -> Option<&mut Doc> {
        self.type_def.doc_mut()
    }

    /// Sets the derives for this enum.
    pub fn derives(&self) -> &[String] {
        self.type_def.derives()
    }

    /// Sets the derives for this enum.
    pub fn set_derives<S>(&mut self, derives: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.type_def.set_derives(derives);
        self
    }

    /// Sets the derives for this enum.
    pub fn with_derives<S>(mut self, derives: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_derives(derives);
        self
    }

    /// Gets a mutable reference to the derives for this enum.
    pub fn derives_mut(&mut self) -> &mut Vec<String> {
        self.type_def.derives_mut()
    }

    /// Pushes a new type that the struct should derive.
    pub fn push_derive(&mut self, derive: impl Into<String>) -> &mut Self {
        self.type_def.push_derive(derive.into());
        self
    }

    /// Pushes a new type that the struct should derive.
    pub fn with_derive(mut self, derive: impl Into<String>) -> Self {
        self.push_derive(derive);
        self
    }

    /// Gets the lints for this enum.
    pub fn lints(&self) -> &[Lint] {
        self.type_def.lints()
    }

    /// Sets the lints for this enum.
    pub fn set_lints<L>(&mut self, lints: impl IntoIterator<Item = L>) -> &mut Self
    where
        L: Into<Lint>,
    {
        self.type_def.set_lints(lints);
        self
    }

    /// Sets the lints for this enum.
    pub fn with_lints<L>(mut self, lints: impl IntoIterator<Item = L>) -> Self
    where
        L: Into<Lint>,
    {
        self.set_lints(lints);
        self
    }

    /// Gets a mutable reference to the lints for this enum.
    pub fn lints_mut(&mut self) -> &mut Vec<Lint> {
        self.type_def.lints_mut()
    }

    /// Pushes a lint attribute to suppress a warning or error.
    pub fn push_lint(&mut self, lint: impl Into<Lint>) -> &mut Self {
        self.type_def.push_lint(lint);
        self
    }

    /// Pushes a lint attribute to suppress a warning or error.
    pub fn with_lint(mut self, lint: impl Into<Lint>) -> Self {
        self.push_lint(lint);
        self
    }

    /// Gets the representation.
    pub fn repr(&self) -> Option<&String> {
        self.type_def.repr()
    }

    /// Sets the representation.
    pub fn set_repr(&mut self, repr: impl Into<Option<String>>) -> &mut Self {
        self.type_def.set_repr(repr);
        self
    }

    /// Sets the representation.
    pub fn with_repr(mut self, repr: impl Into<Option<String>>) -> Self {
        self.set_repr(repr);
        self
    }

    /// Gets a mutable reference to the representation.
    pub fn repr_mut(&mut self) -> Option<&mut String> {
        self.type_def.repr_mut()
    }

    /// Gets the macros for this enum.
    pub fn macros(&self) -> &[String] {
        self.type_def.macros()
    }

    /// Sets the macros for this enum.
    pub fn set_macros<S>(&mut self, macros: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.type_def.set_macros(macros);
        self
    }

    /// Sets the macros for this enum.
    pub fn with_macros<S>(mut self, macros: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_macros(macros);
        self
    }

    /// Gets a mutable reference to the macros for this enum.
    pub fn macros_mut(&mut self) -> &mut Vec<String> {
        self.type_def.macros_mut()
    }

    /// Pushes an arbitrary macro.
    pub fn push_macro(&mut self, r#macro: impl Into<String>) -> &mut Self {
        self.type_def.push_macro(r#macro);
        self
    }

    /// Pushes an arbitrary macro.
    pub fn with_macro(mut self, r#macro: impl Into<String>) -> Self {
        self.push_macro(r#macro);
        self
    }

    /// Gets the variants for this enum.
    pub fn variants(&self) -> &[Variant] {
        &self.variants
    }

    /// Sets the variants for this enum.
    pub fn set_variants<V>(&mut self, variants: impl IntoIterator<Item = V>) -> &mut Self
    where
        V: Into<Variant>,
    {
        self.variants = variants.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the variants for this enum.
    pub fn with_variants<V>(mut self, variants: impl IntoIterator<Item = V>) -> Self
    where
        V: Into<Variant>,
    {
        self.set_variants(variants);
        self
    }

    /// Gets a mutable reference to the variants for this enum.
    pub fn variants_mut(&mut self) -> &mut Vec<Variant> {
        &mut self.variants
    }

    /// Pushes a variant to the enum.
    pub fn push_variant(&mut self, variant: impl Into<Variant>) -> &mut Self {
        self.variants.push(variant.into());
        self
    }

    /// Pushes a variant to the enum.
    pub fn with_variant(mut self, variant: impl Into<Variant>) -> Self {
        self.push_variant(variant);
        self
    }

    /// Formats the enum using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.type_def.fmt_head("enum", &[], fmt)?;

        fmt.block(|fmt| {
            for variant in &self.variants {
                variant.fmt(fmt)?;
            }

            Ok(())
        })
    }
}
