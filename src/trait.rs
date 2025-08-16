use std::fmt::{self, Write};

use crate::associated_const::AssociatedConst;
use crate::associated_type::AssociatedType;
use crate::bound::Bound;
use crate::doc::Doc;
use crate::formatter::{Formatter, fmt_bound_rhs};
use crate::function::Function;
use crate::generic_parameter::GenericParameter;
use crate::r#type::Type;
use crate::type_def::TypeDef;
use crate::visibility::Vis;

/// Defines a [trait](https://doc.rust-lang.org/book/ch10-02-traits.html).
#[derive(Clone, Debug)]
pub struct Trait {
    type_def: TypeDef,
    parents: Vec<Type>,
    associated_consts: Vec<AssociatedConst>,
    attributes: Vec<String>,
    associated_types: Vec<AssociatedType>,
    functions: Vec<Function>,
}

impl Trait {
    /// Creates a trait definition with the provided name.
    pub fn new(name: impl Into<String>) -> Self {
        Trait {
            type_def: TypeDef::new(name.into()),
            parents: Vec::new(),
            associated_consts: Vec::new(),
            attributes: Vec::new(),
            associated_types: Vec::new(),
            functions: Vec::new(),
        }
    }

    /// Gets the name of the trait.
    pub fn name(&self) -> &str {
        self.type_def.ty().name()
    }

    /// Sets the name of the trait.
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.type_def.ty_mut().set_name(name);
        self
    }

    /// Sets the name of the trait.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }

    /// Gets a mutable reference to the name of the trait.
    pub fn name_mut(&mut self) -> &mut String {
        self.type_def.ty_mut().name_mut()
    }

    /// Gets the trait visibility.
    pub fn vis(&mut self) -> &Vis {
        self.type_def.vis()
    }

    /// Sets the trait visibility.
    pub fn set_vis(&mut self, vis: impl Into<Vis>) -> &mut Self {
        self.type_def.set_vis(vis.into());
        self
    }

    /// Sets the trait visibility.
    pub fn with_vis(mut self, vis: impl Into<Vis>) -> Self {
        self.set_vis(vis);
        self
    }

    /// Gets a mutable reference to the trait visibility.
    pub fn vis_mut(&mut self) -> &mut Vis {
        self.type_def.vis_mut()
    }

    /// Gets the attributes.
    pub fn attributes(&self) -> &[String] {
        &self.attributes
    }

    /// Sets the attributes.
    pub fn set_attributes<S>(&mut self, attributes: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.attributes = attributes.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the attributes.
    pub fn with_attributes<S>(mut self, attributes: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_attributes(attributes);
        self
    }

    /// Gets a mutable reference to the attributes.
    pub fn attributes_mut(&mut self) -> &mut Vec<String> {
        &mut self.attributes
    }

    /// Pushes an attribute.
    pub fn push_attribute(&mut self, attr: impl Into<String>) -> &mut Self {
        self.attributes.push(attr.into());
        self
    }

    /// Pushes an attribute.
    pub fn with_attribute(&mut self, attr: impl Into<String>) -> &mut Self {
        self.push_attribute(attr);
        self
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

    /// Returns a mutable reference to the generics.
    pub fn generics_mut(&mut self) -> &mut Vec<GenericParameter> {
        self.type_def.ty_mut().generics_mut()
    }

    /// Pushes a generic to the trait.
    pub fn push_generic(&mut self, generic: impl Into<String>) -> &mut Self {
        self.type_def.ty_mut().push_generic(generic.into());
        self
    }

    /// pushes a generic to the trait.
    pub fn with_generic(mut self, generic: impl Into<String>) -> Self {
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

    /// Pushes a `where` bound to the trait.
    pub fn push_bound(&mut self, bound: impl Into<Bound>) -> &mut Self {
        self.type_def.push_bound(bound.into());
        self
    }

    /// Pushes a `where` bound to the trait.
    pub fn with_bound(mut self, bound: impl Into<Bound>) -> Self {
        self.push_bound(bound);
        self
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

    /// Pushes a macro to the trait def (e.g. `"#[async_trait]"`).
    pub fn push_macro(&mut self, r#macro: impl Into<String>) -> &mut Self {
        self.type_def.push_macro(r#macro.into());
        self
    }

    /// Pushes a macro to the trait def (e.g. `"#[async_trait]"`).
    pub fn with_macro(mut self, r#macro: impl Into<String>) -> Self {
        self.push_macro(r#macro);
        self
    }

    /// Gets the parent traits.
    pub fn parents(&self) -> &[Type] {
        &self.parents
    }

    /// Sets the parent traits.
    pub fn set_parents<T>(&mut self, parents: impl IntoIterator<Item = T>) -> &mut Self
    where
        T: Into<Type>,
    {
        self.parents = parents.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the parent traits.
    pub fn with_parents<T>(mut self, parents: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<Type>,
    {
        self.set_parents(parents);
        self
    }

    /// Gets a mutable reference to the parent traits.
    pub fn parents_mut(&mut self) -> &mut Vec<Type> {
        &mut self.parents
    }

    /// Pushes a parent trait.
    pub fn push_parent(&mut self, parent: impl Into<Type>) -> &mut Self {
        self.parents.push(parent.into());
        self
    }

    /// Pushes a parent trait.
    pub fn with_parent(&mut self, parent: impl Into<Type>) -> &mut Self {
        self.push_parent(parent);
        self
    }

    /// Gets the trait documentation.
    pub fn doc(&mut self) -> Option<&Doc> {
        self.type_def.doc()
    }

    /// Sets the trait documentation.
    pub fn set_doc<S>(&mut self, doc: impl Into<Option<S>>) -> &mut Self
    where
        S: Into<Doc>,
    {
        self.type_def.set_doc(doc);
        self
    }

    /// Sets the trait documentation.
    pub fn with_doc(mut self, doc: impl Into<Doc>) -> Self {
        self.set_doc(doc);
        self
    }

    /// Gets a mutable reference eto the trait documentation.
    pub fn doc_mut(&mut self) -> Option<&mut Doc> {
        self.type_def.doc_mut()
    }

    /// Gets the associated consts.
    pub fn associated_consts(&self) -> &[AssociatedConst] {
        &self.associated_consts
    }

    /// Sets the associated consts.
    pub fn set_associated_consts<C>(
        &mut self,
        associated_consts: impl IntoIterator<Item = C>,
    ) -> &mut Self
    where
        C: Into<AssociatedConst>,
    {
        self.associated_consts = associated_consts.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the associated consts.
    pub fn with_associated_consts<C>(
        mut self,
        associated_consts: impl IntoIterator<Item = C>,
    ) -> Self
    where
        C: Into<AssociatedConst>,
    {
        self.set_associated_consts(associated_consts);
        self
    }

    /// Gets a mutable reference to the associated consts.
    pub fn associated_consts_mut(&mut self) -> &mut Vec<AssociatedConst> {
        &mut self.associated_consts
    }

    /// Pushes an associated const.
    pub fn push_associated_const(
        &mut self,
        associated_const: impl Into<AssociatedConst>,
    ) -> &mut Self {
        self.associated_consts.push(associated_const.into());
        self
    }

    /// Pushes an associated const.
    pub fn with_associated_const(mut self, associated_const: impl Into<AssociatedConst>) -> Self {
        self.push_associated_const(associated_const);
        self
    }

    /// Gets the associated consts.
    pub fn associated_type(&self) -> &[AssociatedType] {
        &self.associated_types
    }

    /// Sets the associated consts.
    pub fn set_associated_types<T>(
        &mut self,
        associated_types: impl IntoIterator<Item = T>,
    ) -> &mut Self
    where
        T: Into<AssociatedType>,
    {
        self.associated_types = associated_types.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the associated consts.
    pub fn with_associated_types<T>(
        mut self,
        associated_types: impl IntoIterator<Item = T>,
    ) -> Self
    where
        T: Into<AssociatedType>,
    {
        self.set_associated_types(associated_types);
        self
    }

    /// Gets a mutable reference to the associated consts.
    pub fn associated_types_mut(&mut self) -> &mut Vec<AssociatedType> {
        &mut self.associated_types
    }

    /// Pushes an associated type.
    pub fn push_associated_type(
        &mut self,
        associated_type: impl Into<AssociatedType>,
    ) -> &mut Self {
        self.associated_types.push(associated_type.into());
        self
    }

    /// Pushes an associated type.
    pub fn with_associated_type(
        &mut self,
        associated_type: impl Into<AssociatedType>,
    ) -> &mut Self {
        self.push_associated_type(associated_type);
        self
    }

    /// Gets the functions.
    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    /// Sets the functions.
    pub fn set_functions<F>(&mut self, functions: impl IntoIterator<Item = F>) -> &mut Self
    where
        F: Into<Function>,
    {
        self.functions = functions.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the functions.
    pub fn with_functions<F>(mut self, functions: impl IntoIterator<Item = F>) -> Self
    where
        F: Into<Function>,
    {
        self.set_functions(functions);
        self
    }

    /// Gets a mutable reference to the functions.
    pub fn functions_mut(&mut self) -> &mut Vec<Function> {
        &mut self.functions
    }

    /// Pushes a function definition.
    pub fn push_function(&mut self, function: impl Into<Function>) -> &mut Self {
        self.functions.push(function.into());
        self
    }

    /// Pushes a function definition.
    pub fn with_function(mut self, function: impl Into<Function>) -> Self {
        self.push_function(function);
        self
    }

    /// Formats the trait using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for attr in &self.attributes {
            writeln!(fmt, "#[{}]", attr)?;
        }

        self.type_def.fmt_head("trait", &self.parents, fmt)?;

        fmt.block(|fmt| {
            let assoc_csts = &self.associated_consts;
            let assoc_tys = &self.associated_types;

            // Format associated consts
            if !assoc_csts.is_empty() {
                for cst in assoc_csts {
                    writeln!(fmt, "const {}: {};", cst.name(), cst.ty())?;
                }
            }

            // Format associated types
            if !assoc_tys.is_empty() {
                for ty in assoc_tys {
                    write!(fmt, "type {}", ty.name())?;

                    let bounded_traits = ty.trait_bounds();
                    if !bounded_traits.is_empty() {
                        write!(fmt, ": ")?;
                        fmt_bound_rhs(bounded_traits, fmt)?;
                    }
                    writeln!(fmt, ";")?;
                }
            }

            // Format the function definitions
            for (i, func) in self.functions.iter().enumerate() {
                if i != 0 || !assoc_tys.is_empty() || !assoc_csts.is_empty() {
                    writeln!(fmt)?;
                }

                func.fmt(true, fmt)?;
            }

            Ok(())
        })
    }
}
