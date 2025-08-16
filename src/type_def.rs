use std::fmt::{self, Write};

use crate::bound::Bound;
use crate::doc::Doc;
use crate::formatter::{Formatter, fmt_bounds};
use crate::lint::Lint;
use crate::r#type::Type;
use crate::visibility::Vis;

/// Defines a type definition.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeDef {
    ty: Type,
    vis: Vis,
    doc: Option<Doc>,
    derives: Vec<String>,
    lints: Vec<Lint>,
    attributes: Vec<String>,
    repr: Option<String>,
    bounds: Vec<Bound>,
    macros: Vec<String>,
}

impl TypeDef {
    /// Creates a type definition struct with the provided name.
    pub fn new(name: impl Into<String>) -> Self {
        TypeDef {
            ty: Type::new(name.into()),
            vis: Vis::Private,
            doc: None,
            derives: Vec::new(),
            lints: Vec::new(),
            attributes: Vec::new(),
            repr: None,
            bounds: Vec::new(),
            macros: Vec::new(),
        }
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn set_ty(&mut self, ty: impl Into<Type>) -> &mut Self {
        self.ty = ty.into();
        self
    }

    pub fn with_ty(mut self, ty: impl Into<Type>) -> Self {
        self.set_ty(ty);
        self
    }

    pub fn ty_mut(&mut self) -> &mut Type {
        &mut self.ty
    }

    pub fn vis(&self) -> &Vis {
        &self.vis
    }

    pub fn set_vis(&mut self, vis: impl Into<Vis>) -> &mut Self {
        self.vis = vis.into();
        self
    }

    pub fn with_vis(mut self, vis: impl Into<Vis>) -> Self {
        self.set_vis(vis);
        self
    }

    pub fn vis_mut(&mut self) -> &mut Vis {
        &mut self.vis
    }

    pub fn doc(&self) -> Option<&Doc> {
        self.doc.as_ref()
    }

    pub fn set_doc<S>(&mut self, doc: impl Into<Option<S>>) -> &mut Self
    where
        S: Into<Doc>,
    {
        self.doc = doc.into().map(Into::into);
        self
    }

    pub fn doc_mut(&mut self) -> Option<&mut Doc> {
        self.doc.as_mut()
    }

    pub fn bounds(&self) -> &[Bound] {
        &self.bounds
    }

    pub fn set_bounds<B>(&mut self, bounds: impl IntoIterator<Item = B>) -> &mut Self
    where
        B: Into<Bound>,
    {
        self.bounds = bounds.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_bounds<B>(mut self, bounds: impl IntoIterator<Item = B>) -> Self
    where
        B: Into<Bound>,
    {
        self.set_bounds(bounds);
        self
    }

    pub fn bounds_mut(&mut self) -> &mut Vec<Bound> {
        &mut self.bounds
    }

    pub fn push_bound(&mut self, bound: impl Into<Bound>) -> &mut Self {
        self.bounds.push(bound.into());
        self
    }

    pub fn with_bound(mut self, bound: impl Into<Bound>) -> Self {
        self.push_bound(bound);
        self
    }

    pub fn macros(&self) -> &[String] {
        &self.macros
    }

    pub fn set_macros<S>(&mut self, macros: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.macros = macros.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_macros<S>(mut self, macros: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_macros(macros);
        self
    }

    pub fn macros_mut(&mut self) -> &mut Vec<String> {
        &mut self.macros
    }

    pub fn push_macro(&mut self, r#macro: impl Into<String>) -> &mut Self {
        self.macros.push(r#macro.into());
        self
    }

    pub fn with_macro(mut self, r#macro: impl Into<String>) -> Self {
        self.push_macro(r#macro);
        self
    }

    pub fn attributes(&self) -> &[String] {
        &self.attributes
    }

    pub fn set_attributes<S>(&mut self, attributes: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.attributes = attributes.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_attributes<S>(mut self, attributes: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_attributes(attributes);
        self
    }

    pub fn attributes_mut(&mut self) -> &mut Vec<String> {
        &mut self.attributes
    }

    pub fn push_attribute(&mut self, attribute: impl Into<String>) -> &mut Self {
        self.attributes.push(attribute.into());
        self
    }

    pub fn with_attribute(mut self, attribute: impl Into<String>) -> Self {
        self.push_attribute(attribute);
        self
    }

    pub fn derives(&self) -> &[String] {
        &self.derives
    }

    pub fn set_derives<S>(&mut self, derives: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.derives = derives.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_derives<S>(mut self, derives: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_derives(derives);
        self
    }

    pub fn derives_mut(&mut self) -> &mut Vec<String> {
        &mut self.derives
    }

    pub fn push_derive(&mut self, derive: impl Into<String>) -> &mut Self {
        self.derives.push(derive.into());
        self
    }

    pub fn with_derive(mut self, derive: impl Into<String>) -> Self {
        self.push_derive(derive);
        self
    }

    pub fn lints(&self) -> &[Lint] {
        &self.lints
    }

    pub fn set_lints<L>(&mut self, lints: impl IntoIterator<Item = L>) -> &mut Self
    where
        L: Into<Lint>,
    {
        self.lints = lints.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_lints<L>(mut self, lints: impl IntoIterator<Item = L>) -> Self
    where
        L: Into<Lint>,
    {
        self.set_lints(lints);
        self
    }

    pub fn lints_mut(&mut self) -> &mut Vec<Lint> {
        &mut self.lints
    }

    pub fn push_lint(&mut self, lint: impl Into<Lint>) -> &mut Self {
        self.lints.push(lint.into());
        self
    }

    pub fn with_lint(mut self, lint: impl Into<Lint>) -> Self {
        self.push_lint(lint);
        self
    }

    pub fn repr(&self) -> Option<&String> {
        self.repr.as_ref()
    }

    pub fn set_repr<S>(&mut self, repr: impl Into<Option<S>>) -> &mut Self
    where
        S: Into<String>,
    {
        self.repr = repr.into().map(Into::into);
        self
    }

    pub fn with_repr(mut self, repr: impl Into<Option<String>>) -> Self {
        self.set_repr(repr);
        self
    }

    pub fn repr_mut(&mut self) -> Option<&mut String> {
        self.repr.as_mut()
    }

    pub fn fmt_head(
        &self,
        keyword: &str,
        parents: &[Type],
        fmt: &mut Formatter<'_>,
    ) -> fmt::Result {
        if let Some(ref doc) = self.doc {
            doc.fmt(fmt)?;
        }

        self.fmt_lints(fmt)?;
        self.fmt_derive(fmt)?;
        self.fmt_repr(fmt)?;
        self.fmt_attributes(fmt)?;
        self.fmt_macros(fmt)?;
        self.vis.fmt(fmt)?;

        write!(fmt, "{} ", keyword)?;
        self.ty.fmt(fmt)?;

        if !parents.is_empty() {
            for (i, ty) in parents.iter().enumerate() {
                if i == 0 {
                    write!(fmt, ": ")?;
                } else {
                    write!(fmt, " + ")?;
                }

                ty.fmt(fmt)?;
            }
        }

        fmt_bounds(&self.bounds, fmt)?;

        Ok(())
    }

    fn fmt_attributes(&self, fmt: &mut Formatter) -> fmt::Result {
        for attr in &self.attributes {
            writeln!(fmt, "#[{}]", attr)?;
        }

        Ok(())
    }

    fn fmt_lints(&self, fmt: &mut Formatter) -> fmt::Result {
        for lint in &self.lints {
            lint.fmt(fmt)?;
        }

        Ok(())
    }

    fn fmt_repr(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref repr) = self.repr {
            writeln!(fmt, "#[repr({})]", repr)?;
        }

        Ok(())
    }

    fn fmt_derive(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if !self.derives.is_empty() {
            write!(fmt, "#[derive(")?;

            for (i, name) in self.derives.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ", ")?
                }
                write!(fmt, "{}", name)?;
            }

            writeln!(fmt, ")]")?;
        }

        Ok(())
    }

    fn fmt_macros(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for m in self.macros.iter() {
            writeln!(fmt, "{}", m)?;
        }
        Ok(())
    }
}
