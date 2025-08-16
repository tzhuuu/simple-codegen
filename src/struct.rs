use std::fmt::{self, Write};

use crate::bound::Bound;
use crate::doc::Doc;
use crate::field::Field;
use crate::fields::Fields;
use crate::formatter::Formatter;
use crate::generic_parameter::GenericParameter;
use crate::lint::Lint;
use crate::r#type::Type;
use crate::type_def::TypeDef;
use crate::visibility::Vis;

/// Defines a struct.
#[derive(Clone, Debug)]
pub struct Struct {
    type_def: TypeDef,

    /// Struct fields
    fields: Fields,
}

impl Struct {
    /// Creates a new struct definition with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Struct {
            type_def: TypeDef::new(name.into()),
            fields: Fields::Empty,
        }
    }

    /// Gets the name of the struct.
    pub fn name(&self) -> &str {
        self.type_def.ty().name()
    }

    /// Sets the name of the struct.
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.type_def.set_ty(Type::new(name.into()));
        self
    }

    /// Sets the name of the struct.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }

    /// Gets a mutable reference to the name of the struct.
    pub fn name_mut(&mut self) -> &mut String {
        self.type_def.ty_mut().name_mut()
    }

    /// Gets the visibility of the struct.
    pub fn vis(&self) -> &Vis {
        self.type_def.vis()
    }

    /// Sets the visibility of the struct.
    pub fn set_vis(&mut self, vis: impl Into<Vis>) -> &mut Self {
        self.type_def.set_vis(vis.into());
        self
    }

    /// Sets the visibility of the struct.
    pub fn with_vis(mut self, vis: impl Into<Vis>) -> Self {
        self.set_vis(vis);
        self
    }

    /// Gets a mutable reference to the visibility of the struct.
    pub fn vis_mut(&mut self) -> &mut Vis {
        self.type_def.vis_mut()
    }

    /// Gets the generic parameters of the struct.
    pub fn generics(&self) -> &[GenericParameter] {
        self.type_def.ty().generics()
    }

    /// Sets the generic parameters of the struct.
    pub fn set_generics<G>(&mut self, generics: impl IntoIterator<Item = G>) -> &mut Self
    where
        G: Into<GenericParameter>,
    {
        self.type_def.ty_mut().set_generics(generics);
        self
    }

    /// Sets the generic parameters of the struct.
    pub fn with_generics<G>(&mut self, generics: impl IntoIterator<Item = G>) -> &mut Self
    where
        G: Into<GenericParameter>,
    {
        self.set_generics(generics);
        self
    }

    /// Gets a mutable reference to the generics of the struct.
    pub fn generics_mut(&mut self) -> &mut Vec<GenericParameter> {
        self.type_def.ty_mut().generics_mut()
    }

    /// Pushes a generic to the struct.
    pub fn push_generic(&mut self, generic: impl Into<GenericParameter>) -> &mut Self {
        self.type_def.ty_mut().push_generic(generic);
        self
    }

    /// Pushes a generic to the struct.
    pub fn with_generic(mut self, generic: impl Into<GenericParameter>) -> Self {
        self.push_generic(generic);
        self
    }

    /// Gets the bounds of the struct.
    pub fn bounds(&self) -> &[Bound] {
        self.type_def.bounds()
    }

    /// Sets the bounds of the struct.
    pub fn set_bounds<B>(&mut self, bounds: impl IntoIterator<Item = B>) -> &mut Self
    where
        B: Into<Bound>,
    {
        self.type_def.set_bounds(bounds);
        self
    }

    /// Sets the bounds of the struct.
    pub fn with_bounds<B>(mut self, bounds: impl IntoIterator<Item = B>) -> Self
    where
        B: Into<Bound>,
    {
        self.set_bounds(bounds);
        self
    }

    /// Gets a mutable reference to the bounds of the struct.
    pub fn bounds_mut(&mut self) -> &mut Vec<Bound> {
        self.type_def.bounds_mut()
    }

    /// Pushes a `where` bound to the struct.
    pub fn push_bound(&mut self, bound: impl Into<Bound>) -> &mut Self {
        self.type_def.push_bound(bound.into());
        self
    }

    /// Pushes a `where` bound to the struct.
    pub fn with_bound(mut self, bound: impl Into<Bound>) -> Self {
        self.push_bound(bound);
        self
    }

    /// Gets the struct documentation.
    pub fn doc(&self) -> Option<&Doc> {
        self.type_def.doc()
    }

    /// Sets the struct documentation.
    pub fn set_doc<S>(&mut self, doc: impl Into<Option<S>>) -> &mut Self
    where
        S: Into<Doc>,
    {
        self.type_def.set_doc(doc);
        self
    }

    /// Sets the struct documentation.
    pub fn with_doc<S>(&mut self, doc: impl Into<Option<S>>) -> &mut Self
    where
        S: Into<Doc>,
    {
        self.set_doc(doc);
        self
    }

    /// Gets a mutable reference to the struct documentation.
    pub fn doc_mut(&mut self) -> Option<&mut Doc> {
        self.type_def.doc_mut()
    }

    /// Gets the derives of the struct.
    pub fn derives(&self) -> &[String] {
        self.type_def.derives()
    }

    /// Sets the derives of the struct.
    pub fn set_derives<S>(&mut self, derives: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.type_def.set_derives(derives);
        self
    }

    /// Sets the derives of the struct.
    pub fn with_derives<S>(mut self, derives: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_derives(derives);
        self
    }

    /// Gets a mutable reference to the derives of the struct.
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

    /// Gets the attributes of the struct.
    pub fn attributes(&self) -> &[String] {
        self.type_def.attributes()
    }

    /// Sets the attributes of the struct.
    pub fn set_attributes<A>(&mut self, attributes: impl IntoIterator<Item = A>) -> &mut Self
    where
        A: Into<String>,
    {
        self.type_def.set_attributes(attributes);
        self
    }

    /// Sets the attributes of the struct.
    pub fn with_attributes<A>(mut self, attributes: impl IntoIterator<Item = A>) -> Self
    where
        A: Into<String>,
    {
        self.set_attributes(attributes);
        self
    }

    /// Gets a mutable reference to the attributes of the struct.
    pub fn attributes_mut(&mut self) -> &mut Vec<String> {
        self.type_def.attributes_mut()
    }

    /// Pushes a new attribute to the struct.
    pub fn push_attribute(&mut self, attribute: impl Into<String>) -> &mut Self {
        self.type_def.push_attribute(attribute.into());
        self
    }

    /// Pushes a new attribute to the struct.
    pub fn with_attribute(mut self, attribute: impl Into<String>) -> Self {
        self.push_attribute(attribute);
        self
    }

    /// Gets the lints of the struct.
    pub fn lints(&self) -> &[Lint] {
        self.type_def.lints()
    }

    /// Sets the lints of the struct.
    pub fn set_lints<L>(&mut self, lints: impl IntoIterator<Item = L>) -> &mut Self
    where
        L: Into<Lint>,
    {
        self.type_def.set_lints(lints);
        self
    }

    /// Sets the lints of the struct.
    pub fn with_lints<L>(mut self, lints: impl IntoIterator<Item = L>) -> Self
    where
        L: Into<Lint>,
    {
        self.set_lints(lints);
        self
    }

    /// Gets a mutable reference to the lints of the struct.
    pub fn lints_mut(&mut self) -> &mut Vec<Lint> {
        self.type_def.lints_mut()
    }

    /// Pushes a lint attribute to suppress a warning or error.
    pub fn push_lint(&mut self, lint: impl Into<Lint>) -> &mut Self {
        self.type_def.push_lint(lint.into());
        self
    }

    /// Pushes a lint attribute to suppress a warning or error.
    pub fn with_lint(mut self, lint: impl Into<Lint>) -> Self {
        self.push_lint(lint);
        self
    }

    /// Gets the representation.
    pub fn repr(&mut self) -> Option<&String> {
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

    /// Gets the macros.
    pub fn macros(&self) -> &[String] {
        self.type_def.macros()
    }

    /// Sets the macros.
    pub fn set_macros<S>(&mut self, macros: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.type_def.set_macros(macros);
        self
    }

    /// Sets the macros.
    pub fn with_macros<S>(mut self, macros: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_macros(macros);
        self
    }

    /// Gets a mutable reference to the macros.
    pub fn macros_mut(&mut self) -> &mut Vec<String> {
        self.type_def.macros_mut()
    }

    /// Pushes an arbitrary macro.
    pub fn push_macro(&mut self, r#macro: impl Into<String>) -> &mut Self {
        self.type_def.push_macro(r#macro.into());
        self
    }

    /// Pushes an arbitrary macro.
    pub fn with_macro(mut self, r#macro: impl Into<String>) -> Self {
        self.push_macro(r#macro);
        self
    }

    /// Gets the fields.
    pub fn fields(&self) -> &Fields {
        &self.fields
    }

    /// Sets the fields.
    pub fn set_fields(&mut self, fields: impl Into<Fields>) -> &mut Self {
        self.fields = fields.into();
        self
    }

    /// Sets the fields.
    pub fn with_fields(mut self, fields: impl Into<Fields>) -> Self {
        self.set_fields(fields);
        self
    }

    /// Gets a mutable reference to the fields.
    pub fn fields_mut(&mut self) -> &mut Fields {
        &mut self.fields
    }

    /// Pushes a named field to the struct.
    ///
    /// A struct can either set named fields with this function or tuple fields
    /// with `push_tuple_field`, but not both.
    pub fn push_named_field(&mut self, named_field: Field) -> &mut Self {
        self.fields.push_named(named_field);
        self
    }

    /// Pushes a named field to the struct.
    ///
    /// A struct can either set named fields with this function or tuple fields
    /// with `push_tuple_field`, but not both.
    pub fn with_named_field(mut self, named_field: Field) -> Self {
        self.push_named_field(named_field);
        self
    }

    /// Pushes a tuple field to the struct.
    ///
    /// A struct can either set tuple fields with this function or named fields
    /// with `field`, but not both.
    pub fn push_tuple_field(&mut self, tuple_field: impl Into<Type>) -> &mut Self {
        self.fields.push_tuple(tuple_field.into());
        self
    }

    // Pushes a tuple field to the struct.
    ///
    /// A struct can either set tuple fields with this function or named fields
    /// with `field`, but not both.
    pub fn with_tuple_field(mut self, tuple_field: impl Into<Type>) -> Self {
        self.push_tuple_field(tuple_field);
        self
    }

    /// Formats the struct using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.type_def.fmt_head("struct", &[], fmt)?;
        self.fields.fmt(fmt)?;

        match self.fields {
            Fields::Empty => {
                writeln!(fmt, ";")?;
            }
            Fields::Tuple(..) => {
                writeln!(fmt, ";")?;
            }
            _ => {}
        }

        Ok(())
    }
}
