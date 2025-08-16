use std::fmt::{self, Write};

use crate::block::Block;
use crate::body::Body;
use crate::bound::Bound;
use crate::doc::Doc;
use crate::field::Field;
use crate::formatter::{Formatter, fmt_bounds, fmt_generics};
use crate::lint::Lint;
use crate::r#type::Type;
use crate::visibility::Vis;

/// Defines a [function](https://doc.rust-lang.org/rust-by-example/fn.html).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Function {
    /// Name of the function
    name: String,

    /// Function documentation
    doc: Option<Doc>,

    /// A lint attribute used to suppress a warning or error
    lints: Vec<Lint>,

    /// Function visibility
    vis: Vis,

    /// Whether or not this function is `async` or not
    r#async: bool,

    /// Function generics
    generics: Vec<String>,

    /// If the function takes `&self` or `&mut self`
    self_arg: SelfArg,

    /// Function arguments
    args: Vec<Field>,

    /// Return type
    ret: Option<Type>,

    /// Where bounds
    bounds: Vec<Bound>,

    /// Body contents
    body: Vec<Body>,

    /// Function attributes, e.g., `#[no_mangle]`.
    attributes: Vec<String>,

    /// Function `extern` ABI
    extern_abi: Option<String>,
}

impl Function {
    /// Return a new function definition.
    pub fn new(name: impl Into<String>) -> Self {
        Function {
            name: name.into(),
            doc: None,
            lints: Vec::new(),
            vis: Vis::Private,
            r#async: false,
            generics: Vec::new(),
            self_arg: SelfArg::None,
            args: Vec::new(),
            ret: None,
            bounds: Vec::new(),
            body: Vec::new(),
            attributes: Vec::new(),
            extern_abi: None,
        }
    }

    /// Gets the function name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the function name.
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }

    /// Sets the function name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }

    /// Gets a mutable reference to the function name.
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Gets the function documentation.
    pub fn doc(&self) -> Option<&Doc> {
        self.doc.as_ref()
    }

    /// Sets the function documentation.
    pub fn set_doc<S>(&mut self, doc: impl Into<Option<S>>) -> &mut Self
    where
        S: Into<Doc>,
    {
        self.doc = doc.into().map(Into::into);
        self
    }

    /// Sets the function documentation.
    pub fn with_doc<S>(mut self, doc: impl Into<Option<S>>) -> Self
    where
        S: Into<Doc>,
    {
        self.set_doc(doc);
        self
    }

    /// Gets a mutable reference to the function documentation.
    pub fn doc_mut(&mut self) -> Option<&mut Doc> {
        self.doc.as_mut()
    }

    /// Gets the lints for the function.
    pub fn lints(&self) -> &[Lint] {
        &self.lints
    }

    /// Sets the lints for the function.
    pub fn set_lints<L>(&mut self, lints: impl IntoIterator<Item = L>) -> &mut Self
    where
        L: Into<Lint>,
    {
        self.lints = lints.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the lints for the function.
    pub fn with_lints<L>(mut self, lints: impl IntoIterator<Item = L>) -> Self
    where
        L: Into<Lint>,
    {
        self.set_lints(lints);
        self
    }

    /// Gets a mutable reference to the lints for the function.
    pub fn lints_mut(&mut self) -> &mut Vec<Lint> {
        &mut self.lints
    }

    /// Pushes a lint for the function.
    pub fn push_lint(&mut self, lint: impl Into<Lint>) -> &mut Self {
        self.lints.push(lint.into());
        self
    }

    /// Pushes a lint for the function.
    pub fn with_lint(mut self, lint: impl Into<Lint>) -> Self {
        self.push_lint(lint);
        self
    }

    /// Gets the function visibility.
    pub fn vis(&self) -> &Vis {
        &self.vis
    }

    /// Sets the function visibility.
    pub fn set_vis(&mut self, vis: impl Into<Vis>) -> &mut Self {
        self.vis = vis.into();
        self
    }

    /// Sets the function visibility.
    pub fn with_vis(mut self, vis: impl Into<Vis>) -> Self {
        self.set_vis(vis);
        self
    }

    /// Gets a mutable reference to the function visibility.
    pub fn vis_mut(&mut self) -> &mut Vis {
        &mut self.vis
    }

    /// Gets whether this function is async or not
    pub fn is_async(&self) -> bool {
        self.r#async
    }

    /// Sets whether this function is async or not
    pub fn set_async(&mut self, r#async: bool) -> &mut Self {
        self.r#async = r#async;
        self
    }

    /// Sets whether this function is async or not
    pub fn with_async(mut self, r#async: bool) -> Self {
        self.set_async(r#async);
        self
    }

    /// Get a mutable reference to whether this function is async or not
    pub fn async_mut(&mut self) -> &mut bool {
        &mut self.r#async
    }

    /// Gets the generics for the function.
    pub fn generics(&self) -> &[String] {
        &self.generics
    }

    /// Sets the generics for the function.
    pub fn set_generics<S>(&mut self, generics: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.generics = generics.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the generics for the function.
    pub fn with_generics<S>(mut self, generics: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_generics(generics);
        self
    }

    /// Gets a mutable reference to the generics attached to the function.
    pub fn generics_mut(&mut self) -> &mut Vec<String> {
        &mut self.generics
    }

    /// Pushes a generic to the function.
    pub fn push_generic(&mut self, ty: impl Into<String>) -> &mut Self {
        self.generics.push(ty.into());
        self
    }

    /// Pushes a generic to the type.
    pub fn with_generic(mut self, ty: impl Into<String>) -> Self {
        self.push_generic(ty);
        self
    }

    /// Gets the `self` argument type.
    pub fn self_arg(&self) -> &SelfArg {
        &self.self_arg
    }

    /// Sets the `self` argument type.
    pub fn set_self_arg(&mut self, self_arg: impl Into<SelfArg>) -> &mut Self {
        self.self_arg = self_arg.into();
        self
    }

    /// Sets the `self` argument type.
    pub fn with_self_arg(mut self, self_arg: impl Into<SelfArg>) -> Self {
        self.set_self_arg(self_arg);
        self
    }

    /// Gets a mutable reference to the `self` argument type.
    pub fn self_arg_mut(&mut self) -> &mut SelfArg {
        &mut self.self_arg
    }

    /// Gets the function arguments.
    pub fn args(&self) -> &[Field] {
        &self.args
    }

    /// Sets the function arguments.
    pub fn set_args<F>(&mut self, args: impl IntoIterator<Item = F>) -> &mut Self
    where
        F: Into<Field>,
    {
        self.args = args.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the function arguments.
    pub fn with_args<F>(mut self, args: impl IntoIterator<Item = F>) -> Self
    where
        F: Into<Field>,
    {
        self.set_args(args);
        self
    }

    /// Gets a mutable reference to the function arguments.
    pub fn args_mut(&mut self) -> &mut Vec<Field> {
        &mut self.args
    }

    /// Pushes a function argument.
    pub fn push_arg(&mut self, name: impl Into<String>, ty: impl Into<Type>) -> &mut Self {
        // While a `Field` is used here, both `documentation`, `visibility`
        // and `annotation` does not make sense for function arguments.
        // Simply use empty strings.
        let f = Field::new(name.into(), ty.into());
        self.args.push(f);
        self
    }

    /// Pushes a function argument.
    pub fn with_arg(mut self, name: impl Into<String>, ty: impl Into<Type>) -> Self {
        self.push_arg(name, ty);
        self
    }

    /// Sets the function return type.
    pub fn ret(&self) -> Option<&Type> {
        self.ret.as_ref()
    }

    /// Sets the function return type.
    pub fn set_ret(&mut self, ty: impl Into<Type>) -> &mut Self {
        self.ret = Some(ty.into());
        self
    }

    /// Sets the function return type.
    pub fn with_ret(mut self, ty: impl Into<Type>) -> Self {
        self.set_ret(ty);
        self
    }

    /// Gets a mutable reference to the function return type.
    pub fn ret_mut(&mut self) -> Option<&mut Type> {
        self.ret.as_mut()
    }

    /// Gets the bounds of the function.
    pub fn bounds(&self) -> &[Bound] {
        &self.bounds
    }

    /// Sets the bounds of the function.
    pub fn set_bounds<B>(&mut self, bounds: impl IntoIterator<Item = B>) -> &mut Self
    where
        B: Into<Bound>,
    {
        self.bounds = bounds.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the bounds of the function.
    pub fn with_bounds<B>(mut self, bounds: impl IntoIterator<Item = B>) -> Self
    where
        B: Into<Bound>,
    {
        self.set_bounds(bounds);
        self
    }

    /// Gets a mutable reference to the bounds of the function.
    pub fn bounds_mut(&mut self) -> &mut Vec<Bound> {
        &mut self.bounds
    }

    /// Pushes a `where` bound to the function.
    pub fn push_bound(&mut self, bound: impl Into<Bound>) -> &mut Self {
        self.bounds.push(bound.into());
        self
    }

    /// Pushes a `where` bound to the function.
    pub fn with_bound(mut self, bound: impl Into<Bound>) -> Self {
        self.push_bound(bound);
        self
    }

    /// Gets the body of the function.
    pub fn body(&self) -> &[Body] {
        &self.body
    }

    /// Sets the body of the function.
    pub fn set_body<B>(&mut self, body: impl IntoIterator<Item = B>) -> &mut Self
    where
        B: Into<Body>,
    {
        self.body = body.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the body of the function.
    pub fn with_body<B>(&mut self, body: impl IntoIterator<Item = B>) -> &mut Self
    where
        B: Into<Body>,
    {
        self.set_body(body);
        self
    }

    /// Gets a mutable reference to the body of the function.
    pub fn body_mut(&mut self) -> &mut Vec<Body> {
        &mut self.body
    }

    /// Pushes a line to the function implementation.
    pub fn push_line(&mut self, line: impl Into<String>) -> &mut Self {
        self.body.push(Body::String(line.into()));
        self
    }

    /// Pushes a line to the function implementation.
    pub fn with_line(mut self, line: impl Into<String>) -> Self {
        self.push_line(line);
        self
    }

    /// Pushes a block to the function implementation
    pub fn push_block(&mut self, block: impl Into<Block>) -> &mut Self {
        self.body.push(Body::Block(block.into()));
        self
    }

    /// Pushes a block to the function implementation
    pub fn with_block(mut self, block: impl Into<Block>) -> Self {
        self.push_block(block);
        self
    }

    /// Gets the attributes for the function.
    pub fn attributes(&self) -> &[String] {
        &self.attributes
    }

    /// Sets the attributes for the function.
    pub fn set_attributes<S>(&mut self, attributes: impl IntoIterator<Item = S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.attributes = attributes.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the attributes for the function.
    pub fn with_attributes<S>(mut self, attributes: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.set_attributes(attributes);
        self
    }

    /// Gets a mutable reference to the attributes for the function.
    pub fn attributes_mut(&mut self) -> &mut Vec<String> {
        &mut self.attributes
    }

    /// Pushes an attribute to the function.
    pub fn push_attribute(&mut self, attribute: impl Into<String>) -> &mut Self {
        self.attributes.push(attribute.into());
        self
    }

    /// Pushes an attribute to the function.
    pub fn with_attribute(mut self, attribute: impl Into<String>) -> Self {
        self.push_attribute(attribute);
        self
    }

    /// Gets the `extern` ABI for the function.
    pub fn extern_abi(&self) -> Option<&String> {
        self.extern_abi.as_ref()
    }

    /// Sets an `extern` ABI for the function.
    pub fn set_extern_abi(&mut self, abi: impl Into<String>) -> &mut Self {
        self.extern_abi.replace(abi.into());
        self
    }

    /// Sets an `extern` ABI for the function.
    pub fn with_extern_abi(mut self, abi: impl Into<String>) -> Self {
        self.set_extern_abi(abi);
        self
    }

    /// Gets a mutable reference to the `extern` ABI for the function.
    pub fn extern_abi_mut(&mut self) -> Option<&mut String> {
        self.extern_abi.as_mut()
    }

    /// Formats the function using the given formatter.
    pub fn fmt(&self, is_trait: bool, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref doc) = self.doc {
            doc.fmt(fmt)?;
        }

        for lint in self.lints.iter() {
            lint.fmt(fmt)?;
        }

        for attr in self.attributes.iter() {
            writeln!(fmt, "#[{}]", attr)?;
        }

        if is_trait {
            assert!(
                self.vis == Vis::Private,
                "trait functions do not have visibility modifiers"
            );
        } else {
            self.vis.fmt(fmt)?;
        }

        if let Some(ref extern_abi) = self.extern_abi {
            write!(fmt, "extern \"{extern_abi}\" ", extern_abi = extern_abi)?;
        }

        if self.r#async {
            write!(fmt, "async ")?;
        }

        write!(fmt, "fn {}", self.name)?;
        fmt_generics(&self.generics, fmt)?;

        write!(fmt, "(")?;

        match self.self_arg {
            SelfArg::None => {}
            SelfArg::WithSelf => {
                write!(fmt, "self")?;
            }
            SelfArg::WithSelfRef => {
                write!(fmt, "&self")?;
            }
            SelfArg::WithMutSelf => {
                write!(fmt, "mut self")?;
            }
            SelfArg::WithMutSelfRef => {
                write!(fmt, "&mut self")?;
            }
        }

        for (i, arg) in self.args.iter().enumerate() {
            if i != 0 || self.self_arg != SelfArg::None {
                write!(fmt, ", ")?;
            }

            write!(fmt, "{}: ", arg.name())?;
            arg.ty().fmt(fmt)?;
        }

        write!(fmt, ")")?;

        if let Some(ref ret) = self.ret {
            write!(fmt, " -> ")?;
            ret.fmt(fmt)?;
        }

        fmt_bounds(&self.bounds, fmt)?;

        if self.body.is_empty() {
            if !is_trait {
                panic!("impl blocks must define fn bodies");
            }
            writeln!(fmt, ";")
        } else {
            fmt.block(|fmt| {
                for b in self.body.iter() {
                    b.fmt(fmt)?;
                }
                Ok(())
            })
        }
    }
}

/// An enum for whether a function takes in self.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SelfArg {
    /// Corresponds to f()
    None,
    /// Corresponds to f(self)
    WithSelf,
    /// Corresponds to f(&self)
    WithSelfRef,
    /// Corresponds to f(mut self)
    WithMutSelf,
    /// Corresponds to f(&mut self)
    WithMutSelfRef,
}
