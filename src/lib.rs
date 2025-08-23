#![deny(missing_debug_implementations, missing_docs)]

//! Provides a builder API for generating Rust code.
//!
//! The general strategy for using the crate is as follows:
//!
//! 1. Create a `Scope` instance.
//! 2. Use the builder API to add elements to the scope.
//! 3. Call `Scope::to_string()` to get the generated code.
//!
//! For example:
//!
//! ```rust
//! use simple_codegen::{Field, Scope};
//!
//! let mut scope = Scope::new();
//!
//! scope.new_struct("Foo")
//!     .push_derive("Debug")
//!     .push_named_field(Field::new("one", "usize"))
//!     .push_named_field(Field::new("two", "String"));
//!
//! println!("{}", scope.to_string());
//! ```

mod associated_const;
mod associated_type;
mod block;
mod body;
mod bound;
mod doc;
mod field;
mod fields;
mod files {
    pub mod file;
    pub mod library;
}
mod formatter;
mod function;
mod generic_parameter;
mod import;
mod item;
mod lint;
mod module;
mod scope;
mod type_def;
mod variant;
mod visibility;

mod r#enum;
mod r#impl;
mod r#struct;
mod r#trait;
mod r#type;
mod type_alias;

pub use associated_const::*;
pub use associated_type::*;
pub use block::*;
pub use bound::*;
pub use r#enum::*;
pub use field::*;
pub use fields::*;
pub use files::file::*;
pub use files::library::*;
pub use formatter::*;
pub use function::*;
pub use generic_parameter::*;
pub use r#impl::*;
pub use import::*;
pub use lint::*;
pub use module::*;
pub use scope::*;
pub use r#struct::*;
pub use r#trait::*;
pub use r#type::*;
pub use type_alias::*;
pub use variant::*;
pub use visibility::*;
