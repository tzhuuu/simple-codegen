use crate::r#enum::Enum;
use crate::function::Function;
use crate::r#impl::Impl;
use crate::line_break::LineBreak;
use crate::module::Module;
use crate::r#struct::Struct;
use crate::r#trait::Trait;
use crate::r#type_alias::TypeAlias;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Item {
    Module(Module),
    Struct(Struct),
    Function(Function),
    Trait(Trait),
    Enum(Enum),
    Impl(Impl),
    Raw(String),
    TypeAlias(TypeAlias),
    LineBreak(LineBreak),
}

impl From<Module> for Item {
    fn from(value: Module) -> Self {
        Item::Module(value)
    }
}

impl From<Struct> for Item {
    fn from(value: Struct) -> Self {
        Item::Struct(value)
    }
}

impl From<Function> for Item {
    fn from(value: Function) -> Self {
        Item::Function(value)
    }
}

impl From<Trait> for Item {
    fn from(value: Trait) -> Self {
        Item::Trait(value)
    }
}

impl From<Enum> for Item {
    fn from(value: Enum) -> Self {
        Item::Enum(value)
    }
}

impl From<Impl> for Item {
    fn from(value: Impl) -> Self {
        Item::Impl(value)
    }
}

impl From<String> for Item {
    fn from(value: String) -> Self {
        Item::Raw(value)
    }
}

impl From<TypeAlias> for Item {
    fn from(value: TypeAlias) -> Self {
        Item::TypeAlias(value)
    }
}

impl From<LineBreak> for Item {
    fn from(value: LineBreak) -> Self {
        Item::LineBreak(value)
    }
}
