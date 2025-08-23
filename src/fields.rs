use std::fmt::{self, Write};

use crate::field::Field;
use crate::formatter::Formatter;
use crate::r#type::Type;

/// Defines a set of fields.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Fields {
    /// An empty set of fields.
    Empty,

    /// A tuple of types.
    Tuple(Vec<Type>),

    /// A named set of fields.
    Named(Vec<Field>),
}

impl Default for Fields {
    fn default() -> Self {
        Self::new()
    }
}

impl Fields {
    /// Creates an empty set of fields.
    pub fn new() -> Self {
        Fields::Empty
    }

    /// Push a named field.
    pub fn push_named(&mut self, field: impl Into<Field>) -> &mut Self {
        match *self {
            Fields::Empty => {
                *self = Fields::Named(vec![field.into()]);
            }
            Fields::Named(ref mut fields) => {
                fields.push(field.into());
            }
            _ => panic!("field list is named"),
        }

        self
    }

    /// Pushes a named field.
    pub fn with_named(mut self, field: impl Into<Field>) -> Self {
        self.push_named(field);
        self
    }

    /// Pushes a tuple type.
    pub fn push_tuple(&mut self, ty: impl Into<Type>) -> &mut Self {
        match *self {
            Fields::Empty => {
                *self = Fields::Tuple(vec![ty.into()]);
            }
            Fields::Tuple(ref mut fields) => {
                fields.push(ty.into());
            }
            _ => panic!("field list is tuple"),
        }

        self
    }

    /// Pushes a tuple type.
    pub fn with_tuple(mut self, ty: impl Into<Type>) -> Self {
        self.push_tuple(ty);
        self
    }

    /// Formats the fields using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Fields::Named(ref fields) => {
                assert!(!fields.is_empty());

                fmt.block(|fmt| {
                    for f in fields {
                        if let Some(doc) = f.doc() {
                            for l in doc.as_inner().lines() {
                                writeln!(fmt, "/// {}", l)?;
                            }
                        }
                        if !f.annotations().is_empty() {
                            for ann in f.annotations() {
                                writeln!(fmt, "{}", ann)?;
                            }
                        }
                        f.vis().fmt(fmt)?;
                        write!(fmt, "{}: ", f.name())?;
                        f.ty().fmt(fmt)?;
                        writeln!(fmt, ",")?;
                    }

                    Ok(())
                })?;
            }
            Fields::Tuple(ref tys) => {
                assert!(!tys.is_empty());

                write!(fmt, "(")?;

                for (i, ty) in tys.iter().enumerate() {
                    if i != 0 {
                        write!(fmt, ", ")?;
                    }
                    ty.fmt(fmt)?;
                }

                write!(fmt, ")")?;
            }
            Fields::Empty => {}
        }

        Ok(())
    }
}
