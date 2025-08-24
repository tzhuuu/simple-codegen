use simple_codegen::*;

#[test]
fn empty_scope() {
    let scope = Scope::new();
    assert_eq!(scope.to_string(), "");
}

#[test]
fn scope_with_doc() {
    let mut scope = Scope::new();
    scope.set_doc("This is a test scope.");

    let expect = r#"/// This is a test scope."#;

    assert_eq!(scope.to_string(), expect);
}

#[test]
fn scope_with_imports() {
    let mut scope = Scope::new();
    scope
        .push_import("bar", "Bar", Vis::Private)
        .push_import("baz", "Baz", Vis::Private);

    let expect = r#"
use bar::Bar;
use baz::Baz;
"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn scope_with_repeated_import_paths() {
    let mut scope = Scope::new();
    scope
        .push_import("bar", "Bar", Vis::Private)
        .push_import("bar", "Bar2", Vis::Private)
        .push_import("baz", "Baz", Vis::Private);

    let expect = r#"
use bar::{Bar, Bar2};
use baz::Baz;
"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn scope_with_overlapping_import_paths() {
    let mut scope = Scope::new();
    scope
        .push_import("bar", "Bar", Vis::Private)
        .push_import("bar", "Bar2", Vis::Private)
        .push_import("bar::inner", "Bar3", Vis::Private)
        .push_import("baz", "Baz", Vis::Private);

    let expect = r#"
use bar::{Bar, Bar2};
use bar::inner::Bar3;
use baz::Baz;
"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
#[should_panic]
fn scope_with_repeated_new_module() {
    let mut scope = Scope::new();
    scope.new_module("foo");
    scope.new_module("foo");

    // This should panic because the module "foo" already exists.
    scope.to_string();
}

#[test]
fn get_or_new_module() {
    let mut scope = Scope::new();
    assert!(scope.get_module("foo").is_none());

    scope
        .get_or_new_module("foo")
        .push_import("bar", "Bar", Vis::Private);

    scope
        .get_or_new_module("foo")
        .new_struct("Foo")
        .push_named_field(Field::new("bar", "Bar"));

    let expect = r#"
mod foo {
    use bar::Bar;

    struct Foo {
        bar: Bar,
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn module_mut() {
    let mut scope = Scope::new();
    scope.new_module("foo").push_import("bar", "Bar", Vis::Pub);

    scope
        .get_module_mut("foo")
        .expect("module_mut")
        .new_struct("Foo")
        .push_named_field(Field::new("bar", "Bar"));

    let expect = r#"
mod foo {
    pub use bar::Bar;

    struct Foo {
        bar: Bar,
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn two_structs() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .push_named_field(Field::new("one", "usize"))
        .push_named_field(Field::new("two", "String"));

    scope
        .new_struct("Bar")
        .push_named_field(Field::new("hello", "World"));

    let expect = r#"
struct Foo {
    one: usize,
    two: String,
}

struct Bar {
    hello: World,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}
