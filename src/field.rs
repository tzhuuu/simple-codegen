use crate::doc::Doc;
use crate::r#type::Type;
use crate::visibility::Vis;

/// Defines a struct field.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Field {
    /// Field name
    name: String,

    /// Field type
    ty: Type,

    /// Field documentation
    doc: Option<Doc>,

    /// Field annotation
    annotations: Vec<String>,

    /// Field value
    value: String,

    /// The visibility of the field
    vis: Vis,
}

impl Field {
    /// Creates a field definition with the provided name and type
    pub fn new(name: impl Into<String>, ty: impl Into<Type>) -> Self {
        Field {
            name: name.into(),
            ty: ty.into(),
            doc: None,
            annotations: Vec::new(),
            value: String::new(),
            vis: Vis::Private,
        }
    }

    /// Gets the name of the field.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the field.
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }

    /// Sets the name of the field.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }

    /// Gets a mutable reference to the name of the field.
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Gets the type of the field.
    pub fn ty(&self) -> &Type {
        &self.ty
    }

    /// Sets the type of the field.
    pub fn set_ty(&mut self, ty: impl Into<Type>) -> &mut Self {
        self.ty = ty.into();
        self
    }

    /// Sets the type of the field.
    pub fn with_ty(mut self, ty: impl Into<Type>) -> Self {
        self.set_ty(ty);
        self
    }

    /// Gets a mutable reference to the type of the field.
    pub fn ty_mut(&mut self) -> &mut Type {
        &mut self.ty
    }

    /// Gets the documentation for the field.
    pub fn doc(&self) -> Option<&Doc> {
        self.doc.as_ref()
    }

    /// Sets the field documentation.
    pub fn set_doc<S>(&mut self, doc: impl Into<Option<S>>) -> &mut Self
    where
        S: Into<Doc>,
    {
        self.doc = doc.into().map(Into::into);
        self
    }

    /// Sets field's documentation.
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.set_doc(doc);
        self
    }

    /// Gets a mutable reference to the field's documentation.
    pub fn doc_mut(&mut self) -> Option<&mut Doc> {
        self.doc.as_mut()
    }

    /// Gets the annotations for the field.
    pub fn annotations(&self) -> &[String] {
        &self.annotations
    }

    /// Sets field's annotations.
    pub fn set_annotations<S>(&mut self, annotations: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.annotations = annotations.into_iter().map(Into::into).collect();
        self
    }

    /// Sets field's annotations.
    pub fn with_annotations<S>(mut self, annotations: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_annotations(annotations);
        self
    }

    /// Gets a mutable reference to the annotations for the field.
    pub fn annotations_mut(&mut self) -> &mut Vec<String> {
        &mut self.annotations
    }

    /// Pushes a single annotation.
    pub fn push_annotation(&mut self, annotation: impl Into<String>) -> &mut Self {
        self.annotations.push(annotation.into());
        self
    }

    /// Pushes a single annotation.
    pub fn with_annotation(mut self, annotation: impl Into<String>) -> Self {
        self.push_annotation(annotation);
        self
    }

    /// Gets the value of the field.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Sets the value of the field.
    pub fn set_value(&mut self, value: impl Into<String>) -> &mut Self {
        self.value = value.into();
        self
    }

    /// Sets the value of the field.
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.set_value(value);
        self
    }

    /// Gets a mutable reference to the value of the field.
    pub fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }

    /// Gets the visibility of the field.
    pub fn vis(&self) -> &Vis {
        &self.vis
    }

    /// Sets the visibility of the field.
    pub fn set_vis(&mut self, vis: impl Into<Vis>) -> &mut Self {
        self.vis = vis.into();
        self
    }

    /// Set the visibility of the field.
    pub fn with_vis(mut self, vis: impl Into<Vis>) -> Self {
        self.set_vis(vis);
        self
    }

    /// Gets a mutable reference to the visibility of the field.
    pub fn vis_mut(&mut self) -> &mut Vis {
        &mut self.vis
    }
}
