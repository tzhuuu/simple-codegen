use std::fmt::{self, Write};

use indexmap::IndexMap;

use crate::doc::Doc;
use crate::r#enum::Enum;
use crate::formatter::Formatter;
use crate::function::Function;
use crate::r#impl::Impl;
use crate::import::Import;
use crate::lint::Lint;
use crate::scope::Scope;
use crate::r#struct::Struct;
use crate::r#trait::Trait;
use crate::visibility::Vis;

/// Defines a module.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Module {
    /// Module name
    name: String,

    /// Visibility
    vis: Vis,

    /// Module documentation
    doc: Option<Doc>,

    /// Contents of the module
    scope: Scope,

    /// Module attributes, e.g., `#[allow(unused_imports)]`.
    attributes: Vec<String>,

    /// Lint rules, e.g. `#[allow(unused_imports)]`
    lints: Vec<Lint>,
}

impl Module {
    /// Creates a new, blank module
    pub fn new(name: impl Into<String>) -> Self {
        Module {
            name: name.into(),
            vis: Vis::Private,
            doc: None,
            scope: Scope::new(),
            attributes: Vec::new(),
            lints: Vec::new(),
        }
    }

    /// Gets the module name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the module name.
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }

    /// Sets the module name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }

    /// Gets a mutable reference to the module name.
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Gets the module documentation.
    pub fn doc(&self) -> Option<&Doc> {
        self.doc.as_ref()
    }

    /// Sets the module documentation.
    pub fn set_doc<S>(&mut self, doc: impl Into<Option<S>>) -> &mut Self
    where
        S: Into<Doc>,
    {
        self.doc = doc.into().map(Into::into);
        self
    }

    /// Sets the module documentation.
    pub fn with_doc<S>(&mut self, doc: impl Into<Option<S>>) -> &mut Self
    where
        S: Into<Doc>,
    {
        self.set_doc(doc);
        self
    }

    /// Gets a mutable reference to the module documentation.
    pub fn doc_mut(&mut self) -> Option<&mut Doc> {
        self.doc.as_mut()
    }

    /// Gets the module's scope.
    pub fn scope(&self) -> &Scope {
        &self.scope
    }

    /// Sets the module's scope.
    pub fn set_scope(&mut self, scope: impl Into<Scope>) -> &mut Self {
        self.scope = scope.into();
        self
    }

    /// Sets the module's scope.
    pub fn with_scope(mut self, scope: impl Into<Scope>) -> Self {
        self.set_scope(scope);
        self
    }

    /// Gets a mutable reference to the module's scope.
    pub fn scope_mut(&mut self) -> &mut Scope {
        &mut self.scope
    }

    /// Gets the module visibility.
    pub fn vis(&self) -> &Vis {
        &self.vis
    }

    /// Sets the module visibility.
    pub fn set_vis(&mut self, vis: impl Into<Vis>) -> &mut Self {
        self.vis = vis.into();
        self
    }

    /// Sets the module visibility.
    pub fn with_vis(mut self, vis: impl Into<Vis>) -> Self {
        self.set_vis(vis);
        self
    }

    /// Gets a mutable reference to the module visibility.
    pub fn vis_mut(&mut self) -> &mut Vis {
        &mut self.vis
    }

    /// Gets the imported types.
    pub fn imports(&self) -> &IndexMap<String, IndexMap<String, Import>> {
        self.scope.imports()
    }

    /// Sets the imported types.
    pub fn set_imports(
        &mut self,
        imports: impl Into<IndexMap<String, IndexMap<String, Import>>>,
    ) -> &mut Self {
        self.scope.set_imports(imports);
        self
    }

    /// Sets the imported types.
    pub fn with_imports(
        mut self,
        imports: impl Into<IndexMap<String, IndexMap<String, Import>>>,
    ) -> Self {
        self.scope.set_imports(imports);
        self
    }

    /// Gets a mutable reference to the imported types.
    pub fn imports_mut(&mut self) -> &mut IndexMap<String, IndexMap<String, Import>> {
        self.scope.imports_mut()
    }

    /// Import a type into the module's scope.
    ///
    /// This results in a new `use` statement being added to the beginning of the
    /// module.
    pub fn push_import(
        &mut self,
        path: impl Into<String>,
        ty: impl Into<String>,
        vis: impl Into<Vis>,
    ) -> &mut Self {
        self.scope.push_import(path, ty, vis);
        self
    }

    /// Import a type into the module's scope.
    ///
    /// This results in a new `use` statement being added to the beginning of the
    /// module.
    pub fn with_import(
        mut self,
        path: impl Into<String>,
        ty: impl Into<String>,
        vis: impl Into<Vis>,
    ) -> Self {
        self.push_import(path, ty, vis);
        self
    }

    /// Gets the attributes for the module.
    pub fn attributes(&self) -> &[String] {
        &self.attributes
    }

    /// Sets the attributes for the module.
    pub fn set_attributes<S>(&mut self, attributes: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.attributes = attributes.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the attributes for the module.
    pub fn with_attributes<S>(mut self, attributes: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_attributes(attributes);
        self
    }

    /// Gets a mutable reference to the attributes for the module.
    pub fn attributes_mut(&mut self) -> &mut Vec<String> {
        &mut self.attributes
    }

    /// Adds an attribute to the module.
    pub fn push_attribute(&mut self, attribute: impl Into<String>) -> &mut Self {
        self.attributes.push(attribute.into());
        self
    }

    /// Adds an attribute to the module.
    pub fn with_attribute(mut self, attribute: impl Into<String>) -> Self {
        self.push_attribute(attribute);
        self
    }

    /// Gets the lints for the module.
    pub fn lints(&self) -> &[Lint] {
        &self.lints
    }

    /// Sets the lints for the module.
    pub fn set_lints<L>(&mut self, lints: impl IntoIterator<Item = L>) -> &mut Self
    where
        L: Into<Lint>,
    {
        self.lints = lints.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the lints for the module.
    pub fn with_lints<L>(mut self, lints: impl IntoIterator<Item = L>) -> Self
    where
        L: Into<Lint>,
    {
        self.set_lints(lints);
        self
    }

    /// Gets a mutable reference to the lints for the module.
    pub fn lints_mut(&mut self) -> &mut Vec<Lint> {
        &mut self.lints
    }

    /// Adds a lint to the module.
    pub fn push_lint(&mut self, lint: impl Into<Lint>) -> &mut Self {
        self.lints.push(lint.into());
        self
    }

    /// Adds a lint to the module.
    pub fn with_lint(mut self, lint: impl Into<Lint>) -> Self {
        self.push_lint(lint);
        self
    }

    /// Pushes a new module definition, returning a mutable reference to it.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it
    /// will return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    pub fn new_module(&mut self, name: impl Into<String>) -> &mut Module {
        self.scope.new_module(name.into())
    }

    /// Gets a reference to a module if it is exists in this scope.
    pub fn get_module<'a>(&self, name: impl Into<&'a str>) -> Option<&Module> {
        self.scope.get_module(name)
    }

    /// Gets a mutable reference to a module if it is exists in this scope.
    pub fn get_module_mut<'a>(&mut self, name: impl Into<&'a str>) -> Option<&mut Module> {
        self.scope.get_module_mut(name)
    }

    /// Gets a mutable reference to a module, creating it if it does
    /// not exist.
    pub fn get_or_new_module<'a>(&mut self, name: impl Into<&'a str>) -> &mut Module {
        self.scope.get_or_new_module(name.into())
    }

    /// Pushes a module definition.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it will
    /// return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    pub fn push_module(&mut self, item: Module) -> &mut Self {
        self.scope.push_module(item);
        self
    }

    /// Pushes a new struct definition, returning a mutable reference to it.
    pub fn new_struct(&mut self, name: impl Into<String>) -> &mut Struct {
        self.scope.new_struct(name.into())
    }

    /// Pushes a structure definition
    pub fn push_struct(&mut self, item: Struct) -> &mut Self {
        self.scope.push_struct(item);
        self
    }

    /// Pushes a new function definition, returning a mutable reference to it.
    pub fn new_function(&mut self, name: impl Into<String>) -> &mut Function {
        self.scope.new_function(name.into())
    }

    /// Pushes a function definition
    pub fn push_function(&mut self, item: Function) -> &mut Self {
        self.scope.push_function(item);
        self
    }

    /// Pushes a new enum definition, returning a mutable reference to it.
    pub fn new_enum(&mut self, name: impl Into<String>) -> &mut Enum {
        self.scope.new_enum(name.into())
    }

    /// Pushes an enum definition
    pub fn push_enum(&mut self, item: Enum) -> &mut Self {
        self.scope.push_enum(item);
        self
    }

    /// Pushes a new `impl` block, returning a mutable reference to it.
    pub fn new_impl(&mut self, target: impl Into<String>) -> &mut Impl {
        self.scope.new_impl(target.into())
    }

    /// Pushes an `impl` block.
    pub fn push_impl(&mut self, item: Impl) -> &mut Self {
        self.scope.push_impl(item);
        self
    }

    /// Pushes a new trait
    pub fn new_trait(&mut self, name: impl Into<String>) -> &mut Trait {
        self.scope.new_trait(name.into())
    }

    /// Pushes a trait definition
    pub fn push_trait(&mut self, item: Trait) -> &mut Self {
        self.scope.push_trait(item);
        self
    }

    /// Formats the module using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref doc) = self.doc {
            doc.fmt(fmt)?;
        }

        for attr in &self.attributes {
            writeln!(fmt, "#[{}] ", attr)?;
        }
        for lint in &self.lints {
            lint.fmt(fmt)?;
        }

        self.vis.fmt(fmt)?;

        write!(fmt, "mod {}", self.name)?;
        fmt.block(|fmt| self.scope.fmt(fmt))
    }
}
