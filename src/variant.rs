use std::fmt::{self, Write};

use crate::field::Field;
use crate::fields::Fields;
use crate::formatter::Formatter;
use crate::r#type::Type;

/// Defines an [enum](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html) variant.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Variant {
    name: String,
    fields: Fields,
    /// Annotations for field e.g., `#[serde(rename = "variant")]`.
    annotations: Vec<String>,
}

impl From<&str> for Variant {
    fn from(name: &str) -> Self {
        Variant::new(name)
    }
}

impl Variant {
    /// Creates a new enum variant with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Variant {
            name: name.into(),
            fields: Fields::Empty,
            annotations: Vec::new(),
        }
    }

    /// Gets the variant's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the variant's name.
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }

    /// Sets the variant's name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }

    /// Gets a mutable reference to the variant's name.
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Gets the variant's fields.
    pub fn fields(&self) -> &Fields {
        &self.fields
    }

    /// Sets the variant's fields.
    pub fn set_fields(&mut self, fields: impl Into<Fields>) -> &mut Self {
        self.fields = fields.into();
        self
    }

    /// Sets the variant's fields.
    pub fn with_fields(mut self, fields: impl Into<Fields>) -> Self {
        self.set_fields(fields);
        self
    }

    /// Gets a mutable reference to the variant's fields.
    pub fn fields_mut(&mut self) -> &mut Fields {
        &mut self.fields
    }

    /// Gets the variant's annotations.
    pub fn annotations(&self) -> &[String] {
        &self.annotations
    }
    /// Sets the variant's annotations.
    pub fn set_annotations<S>(&mut self, annotations: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.annotations = annotations.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the variant's annotations.
    pub fn with_annotations<S>(mut self, annotations: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_annotations(annotations);
        self
    }

    /// Gets a mutable reference to the variant's annotations.
    pub fn annotations_mut(&mut self) -> &mut Vec<String> {
        &mut self.annotations
    }

    /// Pushes an annotation to the variant.
    pub fn push_annotation(&mut self, annotation: impl Into<String>) -> &mut Self {
        self.annotations.push(annotation.into());
        self
    }

    /// Pushes an annotation to the variant.
    pub fn with_annotation(mut self, annotation: impl Into<String>) -> Self {
        self.push_annotation(annotation);
        self
    }

    /// Pushes a named field to the variant.
    ///
    /// Panics if the fields are tuple-based rather than named.
    pub fn push_named_field(&mut self, name: impl Into<String>, ty: impl Into<Type>) -> &mut Self {
        self.fields.push_named(Field::new(name.into(), ty.into()));
        self
    }

    /// Pushes a named field to the variant.
    ///
    /// Panics if the fields are tuple-based rather than named.
    pub fn with_named_field(mut self, name: impl Into<String>, ty: impl Into<Type>) -> Self {
        self.push_named_field(name, ty);
        self
    }

    /// Pushes a tuple field to the variant.
    ///
    /// Panics if the fields are named rather than tuple-based.
    pub fn push_tuple_field(&mut self, ty: impl Into<String>) -> &mut Self {
        self.fields.push_tuple(ty.into());
        self
    }

    /// Pushes a tuple field to the variant.
    ///
    /// Panics if the fields are named rather than tuple-based.
    pub fn with_tuple_field(mut self, ty: impl Into<String>) -> Self {
        self.push_tuple_field(ty);
        self
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for a in &self.annotations {
            write!(fmt, "{}", a)?;
            writeln!(fmt)?;
        }
        write!(fmt, "{}", self.name)?;
        self.fields.fmt(fmt)?;
        writeln!(fmt, ",")?;

        Ok(())
    }
}
