use std::fmt::{self, Debug, Display, Write};

use indexmap::IndexMap;

use crate::doc::Doc;
use crate::r#enum::Enum;
use crate::formatter::Formatter;
use crate::function::Function;
use crate::r#impl::Impl;
use crate::import::Import;
use crate::item::Item;
use crate::line_break::LineBreak;
use crate::module::Module;
use crate::r#struct::Struct;
use crate::r#trait::Trait;
use crate::r#type::Type;
use crate::type_alias::TypeAlias;
use crate::visibility::Vis;

/// Defines a scope.
///
/// A scope contains modules, types, etc...
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Scope {
    /// Scope documentation
    doc: Option<Doc>,

    /// Imports
    imports: IndexMap<String, IndexMap<String, Import>>,

    /// Contents of the documentation,
    items: Vec<Item>,
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        // Remove the trailing newline
        if ret.as_bytes().last() == Some(&b'\n') {
            ret.pop();
        }
        write!(f, "{}", ret)?;
        Ok(())
    }
}

impl Scope {
    /// Creates a new scope.
    pub fn new() -> Self {
        Scope {
            doc: None,
            imports: IndexMap::new(),
            items: Vec::new(),
        }
    }

    /// Gets the scope documentation.
    pub fn doc(&self) -> Option<&Doc> {
        self.doc.as_ref()
    }

    /// Sets the scope documentation.
    pub fn set_doc<S>(&mut self, doc: impl Into<Option<S>>) -> &mut Self
    where
        S: Into<Doc>,
    {
        self.doc = doc.into().map(Into::into);
        self
    }

    /// Sets the scope documentation.
    pub fn with_doc<S>(&mut self, doc: impl Into<Option<S>>) -> &mut Self
    where
        S: Into<Doc>,
    {
        self.set_doc(doc);
        self
    }

    /// Gets a mutable reference to the scope documentation.
    pub fn doc_mut(&mut self) -> Option<&mut Doc> {
        self.doc.as_mut()
    }

    /// Gets the imported types.
    pub fn imports(&self) -> &IndexMap<String, IndexMap<String, Import>> {
        &self.imports
    }

    /// Sets the imported types.
    pub fn set_imports(
        &mut self,
        imports: impl Into<IndexMap<String, IndexMap<String, Import>>>,
    ) -> &mut Self {
        self.imports = imports.into();
        self
    }

    /// Sets the imported types.
    pub fn with_imports(
        mut self,
        imports: impl Into<IndexMap<String, IndexMap<String, Import>>>,
    ) -> Self {
        self.set_imports(imports);
        self
    }

    /// Gets a mutable reference to the imported types.
    pub fn imports_mut(&mut self) -> &mut IndexMap<String, IndexMap<String, Import>> {
        &mut self.imports
    }

    /// Imports a type into the scope.
    ///
    /// This results in a new `use` statement being added to the beginning of
    /// the scope.
    pub fn push_import(
        &mut self,
        path: impl Into<String>,
        ty: impl Into<String>,
        vis: impl Into<Vis>,
    ) -> &mut Self {
        // handle cases where the caller wants to refer to a type namespaced
        // within the containing namespace, like "a::B".
        let ty = ty.into();
        let path = path.into();

        let ty = ty.split("::").next().unwrap_or(ty.as_str());
        self.imports
            .entry(path.clone())
            .or_default()
            .entry(ty.to_string())
            .or_insert_with(|| Import::new(path, ty).with_vis(vis));
        self
    }

    /// Imports a type into the scope.
    ///
    /// This results in a new `use` statement being added to the beginning of
    /// the scope.
    pub fn with_import(
        mut self,
        path: impl Into<String>,
        ty: impl Into<String>,
        vis: impl Into<Vis>,
    ) -> Self {
        self.push_import(path, ty, vis);
        self
    }

    /// Gets the items inside the scope.
    pub fn items(&self) -> &[Item] {
        &self.items
    }

    /// Sets the items inside the scope.
    pub fn set_items<I, T>(&mut self, items: impl Into<I>) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Item>,
    {
        self.items = items.into().into_iter().map(Into::into).collect();
        self
    }

    /// Sets the items inside the scope.
    pub fn with_items<I, T>(mut self, items: impl Into<I>) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Item>,
    {
        self.set_items(items);
        self
    }

    /// Gets a mutable reference to the items inside the scope.
    pub fn items_mut(&mut self) -> &mut Vec<Item> {
        &mut self.items
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
        self.push_module(Module::new(name.into()));

        match *self.items.last_mut().unwrap() {
            Item::Module(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Gets a mutable reference to a module if it is exists in this scope.
    pub fn get_module_mut<'a>(&mut self, name: impl Into<&'a str>) -> Option<&mut Module> {
        let name = name.into();
        self.items
            .iter_mut()
            .filter_map(|item| match item {
                &mut Item::Module(ref mut module) if module.name() == name => Some(module),
                _ => None,
            })
            .next()
    }

    /// Gets a reference to a module if it is exists in this scope.
    pub fn get_module<'a>(&self, name: impl Into<&'a str>) -> Option<&Module> {
        let name = name.into();
        self.items
            .iter()
            .filter_map(|item| match item {
                Item::Module(module) if module.name() == name => Some(module),
                _ => None,
            })
            .next()
    }

    /// Gets a mutable reference to a module, creating it if it does
    /// not exist.
    pub fn get_or_new_module<'a>(&mut self, name: impl Into<&'a str>) -> &mut Module {
        let name = name.into();
        if self.get_module(name).is_some() {
            self.get_module_mut(name).unwrap()
        } else {
            self.new_module(name)
        }
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
    pub fn push_module(&mut self, module: impl Into<Module>) -> &mut Self {
        let module = module.into();
        assert!(self.get_module(module.name()).is_none());
        self.items.push(Item::Module(module));
        self
    }

    /// Pushes a new struct definition, returning a mutable reference to it.
    pub fn new_struct(&mut self, name: impl Into<String>) -> &mut Struct {
        self.push_struct(Struct::new(name.into()));

        match *self.items.last_mut().unwrap() {
            Item::Struct(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Pushes a struct definition
    pub fn push_struct(&mut self, item: impl Into<Struct>) -> &mut Self {
        self.items.push(Item::Struct(item.into()));
        self
    }

    /// Pushes a new function definition, returning a mutable reference to it.
    pub fn new_function(&mut self, name: impl Into<String>) -> &mut Function {
        self.push_function(Function::new(name.into()));

        match *self.items.last_mut().unwrap() {
            Item::Function(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Pushes a function definition
    pub fn push_function(&mut self, item: Function) -> &mut Self {
        self.items.push(Item::Function(item));
        self
    }

    /// Pushes a new trait definition, returning a mutable reference to it.
    pub fn new_trait(&mut self, name: impl Into<String>) -> &mut Trait {
        self.push_trait(Trait::new(name.into()));

        match *self.items.last_mut().unwrap() {
            Item::Trait(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Pushes a trait definition
    pub fn push_trait(&mut self, item: Trait) -> &mut Self {
        self.items.push(Item::Trait(item));
        self
    }

    /// Pushes a new struct definition, returning a mutable reference to it.
    pub fn new_enum(&mut self, name: impl Into<String>) -> &mut Enum {
        self.push_enum(Enum::new(name.into()));

        match *self.items.last_mut().unwrap() {
            Item::Enum(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Pushes a structure definition
    pub fn push_enum(&mut self, item: Enum) -> &mut Self {
        self.items.push(Item::Enum(item));
        self
    }

    /// Pushes a new `impl` block, returning a mutable reference to it.
    pub fn new_impl(&mut self, target: impl Into<Type>) -> &mut Impl {
        self.push_impl(Impl::new(target.into()));

        match *self.items.last_mut().unwrap() {
            Item::Impl(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Pushes an `impl` block.
    pub fn push_impl(&mut self, item: Impl) -> &mut Self {
        self.items.push(Item::Impl(item));
        self
    }

    /// Pushes a raw string to the scope.
    ///
    /// This string will be included verbatim in the formatted string.
    pub fn raw(&mut self, val: impl Into<String>) -> &mut Self {
        self.items.push(Item::Raw(val.into()));
        self
    }

    /// Pushes a new `TypeAlias`, returning a mutable reference to it.
    pub fn new_type_alias(
        &mut self,
        name: impl Into<String>,
        target: impl Into<String>,
    ) -> &mut TypeAlias {
        self.push_type_alias(TypeAlias::new(name.into(), target.into()));

        match *self.items.last_mut().unwrap() {
            Item::TypeAlias(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Pushes a `TypeAlias`.
    pub fn push_type_alias(&mut self, item: TypeAlias) -> &mut Self {
        self.items.push(Item::TypeAlias(item));
        self
    }

    /// Pushes a `LineBreak`.
    pub fn push_line_break(&mut self) -> &mut Self {
        self.items.push(Item::LineBreak(LineBreak::new()));
        self
    }

    /// Formats the scope using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref doc) = self.doc {
            doc.fmt(fmt)?;
        }

        self.fmt_imports(fmt)?;

        if !self.imports.is_empty() {
            writeln!(fmt)?;
        }

        for (i, item) in self.items.iter().enumerate() {
            if i != 0 {
                writeln!(fmt)?;
            }

            match *item {
                Item::Module(ref v) => v.fmt(fmt)?,
                Item::Struct(ref v) => v.fmt(fmt)?,
                Item::Function(ref v) => v.fmt(false, fmt)?,
                Item::Trait(ref v) => v.fmt(fmt)?,
                Item::Enum(ref v) => v.fmt(fmt)?,
                Item::Impl(ref v) => v.fmt(fmt)?,
                Item::Raw(ref v) => {
                    writeln!(fmt, "{}", v)?;
                }
                Item::TypeAlias(ref v) => v.fmt(fmt)?,
                Item::LineBreak(ref v) => v.fmt(fmt)?,
            }
        }

        Ok(())
    }

    fn fmt_imports(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        // First, collect all visibilities
        let mut visibilities = Vec::new();

        for (_, imports) in &self.imports {
            for (_, import) in imports {
                if !visibilities.contains(import.vis()) {
                    visibilities.push(import.vis().clone());
                }
            }
        }

        let mut tys = Vec::new();

        // Loop over all visibilities and format the associated imports
        for vis in &visibilities {
            for (path, imports) in &self.imports {
                tys.clear();

                for (ty, import) in imports {
                    if vis == import.vis() {
                        tys.push(ty);
                    }
                }

                if !tys.is_empty() {
                    vis.fmt(fmt)?;

                    write!(fmt, "use {}::", path)?;

                    #[allow(clippy::comparison_chain)]
                    if tys.len() > 1 {
                        write!(fmt, "{{")?;

                        for (i, ty) in tys.iter().enumerate() {
                            if i != 0 {
                                write!(fmt, ", ")?;
                            }
                            write!(fmt, "{}", ty)?;
                        }

                        writeln!(fmt, "}};")?;
                    } else if tys.len() == 1 {
                        writeln!(fmt, "{};", tys[0])?;
                    }
                }
            }
        }

        Ok(())
    }
}
