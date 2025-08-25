use std::fmt::{self, Write};

use crate::associated_const::AssociatedConst;
use crate::associated_type::AssociatedType;
use crate::bound::Bound;
use crate::formatter::{Formatter, fmt_bounds, fmt_generics};
use crate::function::Function;
use crate::r#type::Type;

/// Defines an impl block.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Impl {
    /// The struct being implemented
    target: Type,

    /// Impl level generics
    generics: Vec<String>,

    /// If implementing a trait
    impl_trait: Option<Type>,

    /// Associated constants
    associated_consts: Vec<AssociatedConst>,

    /// Associated types
    associated_types: Vec<AssociatedType>,

    /// Bounds
    bounds: Vec<Bound>,

    macros: Vec<String>,

    functions: Vec<Function>,
}

impl Impl {
    /// Creates a new impl definition
    pub fn new(target: impl Into<Type>) -> Self {
        Impl {
            target: target.into(),
            generics: Vec::new(),
            impl_trait: None,
            associated_consts: Vec::new(),
            associated_types: Vec::new(),
            bounds: Vec::new(),
            functions: Vec::new(),
            macros: Vec::new(),
        }
    }

    /// Gets the target type of the impl block.
    pub fn target(&self) -> &Type {
        &self.target
    }

    /// Sets the target type of the impl block.
    pub fn set_target(&mut self, target: impl Into<Type>) -> &mut Self {
        self.target = target.into();
        self
    }

    /// Sets the target type of the impl block.
    pub fn with_target(mut self, target: impl Into<Type>) -> Self {
        self.set_target(target);
        self
    }

    /// Gets a mutable reference to the target type of the impl block.
    pub fn target_mut(&mut self) -> &mut Type {
        &mut self.target
    }

    /// Gets the generics for the impl block.
    pub fn generics(&self) -> &[String] {
        &self.generics
    }

    /// Sets the generics for the impl block.
    pub fn set_generics<G>(&mut self, generics: impl IntoIterator<Item = G>) -> &mut Self
    where
        G: Into<String>,
    {
        self.generics = generics.into_iter().map(Into::into).collect();
        self
    }

    /// Gets the generics for the impl block.
    pub fn with_generics<G>(mut self, generics: impl IntoIterator<Item = G>) -> Self
    where
        G: Into<String>,
    {
        self.set_generics(generics);
        self
    }

    /// GEts a mutable reference to the generics for the impl block.
    pub fn generics_mut(&mut self) -> &mut Vec<String> {
        &mut self.generics
    }

    /// Pushes a generic to the impl block.
    ///
    /// This adds the generic for the block (`impl<T>`) and not the target type.
    pub fn push_generic(&mut self, name: impl Into<String>) -> &mut Self {
        self.generics.push(name.into());
        self
    }

    /// Pushes a generic to the impl block.
    ///
    /// This adds the generic for the block (`impl<T>`) and not the target type.
    pub fn with_generic(mut self, name: impl Into<String>) -> Self {
        self.push_generic(name);
        self
    }

    /// Gets the trait that the impl block is implementing, if any.
    pub fn impl_trait(&self) -> Option<&Type> {
        self.impl_trait.as_ref()
    }

    /// Sets the trait that the impl block is implementing.
    pub fn set_impl_trait(&mut self, ty: impl Into<Type>) -> &mut Self {
        self.impl_trait = Some(ty.into());
        self
    }

    /// Sets the trait that the impl block is implementing.
    pub fn with_impl_trait(mut self, ty: impl Into<Type>) -> Self {
        self.set_impl_trait(ty);
        self
    }

    /// Gets a mutable reference to the trait that the impl block is implementing.
    pub fn impl_trait_mut(&mut self) -> Option<&mut Type> {
        self.impl_trait.as_mut()
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
    pub fn with_associated_consts<G>(
        mut self,
        associated_consts: impl IntoIterator<Item = G>,
    ) -> Self
    where
        G: Into<AssociatedConst>,
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
    pub fn with_associated_type(mut self, associated_type: impl Into<AssociatedType>) -> Self {
        self.push_associated_type(associated_type);
        self
    }

    /// Gets the bounds for the impl block.
    pub fn bounds(&self) -> &[Bound] {
        &self.bounds
    }

    /// Sets the bounds for the impl block.
    pub fn set_bounds<B>(&mut self, bounds: impl IntoIterator<Item = B>) -> &mut Self
    where
        B: Into<Bound>,
    {
        self.bounds = bounds.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the bounds for the impl block.
    pub fn with_bounds<B>(mut self, bounds: impl IntoIterator<Item = B>) -> Self
    where
        B: Into<Bound>,
    {
        self.set_bounds(bounds);
        self
    }

    /// Gets a mutable reference to the bounds for the impl block.
    pub fn bounds_mut(&mut self) -> &mut Vec<Bound> {
        &mut self.bounds
    }

    /// Pushes a bound to the impl block.
    pub fn push_bound(&mut self, bound: impl Into<Bound>) -> &mut Self {
        self.bounds.push(bound.into());
        self
    }

    /// Pushes a bound to the impl block.
    pub fn with_bound(mut self, bound: impl Into<Bound>) -> Self {
        self.push_bound(bound);
        self
    }

    /// Gets the macros for the impl block.
    pub fn macros(&self) -> &[String] {
        &self.macros
    }

    /// Sets the macros for the impl block.
    pub fn set_macros<S>(&mut self, macros: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.macros = macros.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the macros for the impl block.
    pub fn with_macros<S>(mut self, macros: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_macros(macros);
        self
    }

    /// Gets a mutable reference to the macros for the impl block.
    pub fn macros_mut(&mut self) -> &mut Vec<String> {
        &mut self.macros
    }

    /// Pushes a macro to the impl block.
    pub fn push_macro(&mut self, r#macro: impl Into<String>) -> &mut Self {
        self.macros.push(r#macro.into());
        self
    }

    /// Pushes a macro to the impl block.
    pub fn with_macro(mut self, r#macro: impl Into<String>) -> Self {
        self.push_macro(r#macro);
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
    pub fn push_function(&mut self, function: Function) -> &mut Self {
        self.functions.push(function);
        self
    }

    /// Pushes a function definition.
    pub fn with_function(mut self, function: Function) -> Self {
        self.push_function(function);
        self
    }

    /// Formats the impl block using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for m in self.macros.iter() {
            writeln!(fmt, "{}", m)?;
        }
        write!(fmt, "impl")?;
        fmt_generics(&self.generics[..], fmt)?;

        if let Some(ref t) = self.impl_trait {
            write!(fmt, " ")?;
            t.fmt(fmt)?;
            write!(fmt, " for")?;
        }

        write!(fmt, " ")?;
        self.target.fmt(fmt)?;

        fmt_bounds(&self.bounds, fmt)?;

        fmt.block(|fmt| {
            // format associated constants
            if !self.associated_consts.is_empty() {
                for cst in &self.associated_consts {
                    assert!(
                        cst.concrete_value().is_some(),
                        "Associated consts must have a concrete value in impl blocks"
                    );
                    cst.concrete_vis().fmt(fmt)?;
                    writeln!(
                        fmt,
                        "const {}: {} = {};",
                        cst.name(),
                        cst.ty(),
                        cst.concrete_value().unwrap(),
                    )?;
                }
            }

            // format associated types
            if !self.associated_types.is_empty() {
                for ty in &self.associated_types {
                    let Some((concrete_name, concrete_generics)) = ty.concrete_ty() else {
                        panic!(
                            "Associated types must have a concrete type in impl blocks: {}",
                            ty.name()
                        );
                    };
                    writeln!(
                        fmt,
                        "type {} = {}{};",
                        ty.name(),
                        concrete_name,
                        if concrete_generics.is_empty() {
                            String::new()
                        } else {
                            format!("<{}>", concrete_generics.join(", "))
                        }
                    )?;
                }
            }

            for (i, func) in self.functions.iter().enumerate() {
                if i != 0 || !self.associated_types.is_empty() {
                    writeln!(fmt)?;
                }

                func.fmt(false, fmt)?;
            }

            Ok(())
        })
    }
}
