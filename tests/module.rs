use simple_codegen::*;

#[test]
fn empty_module() {
    let mut scope = Scope::new();
    scope.new_module("foo");

    let expect = r#"
mod foo {
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn module_basic() {
    let mut scope = Scope::new();
    scope
        .new_module("foo")
        .set_doc("This is a test module.")
        .set_vis(Vis::Pub);
    let expect = r#"
/// This is a test module.
pub mod foo {
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn module_with_imports() {
    let mut scope = Scope::new();
    scope
        .new_module("foo")
        .push_import("bar", "Bar")
        .push_import("baz", "Baz");

    let expect = r#"
mod foo {
    use bar::Bar;
    use baz::Baz;

}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn module_with_repeated_import_paths() {
    let mut scope = Scope::new();
    scope
        .new_module("foo")
        .push_import("bar", "Bar")
        .push_import("bar", "Bar2")
        .push_import("baz", "Baz");

    let expect = r#"
mod foo {
    use bar::{Bar, Bar2};
    use baz::Baz;

}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn module_with_overlapping_import_paths() {
    let mut scope = Scope::new();
    scope
        .new_module("foo")
        .push_import("bar", "Bar")
        .push_import("bar", "Bar2")
        .push_import("bar::inner", "Bar3")
        .push_import("baz", "Baz");

    let expect = r#"
mod foo {
    use bar::{Bar, Bar2};
    use bar::inner::Bar3;
    use baz::Baz;

}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn module_with_attributes() {
    let mut scope = Scope::new();
    scope.new_module("foo").push_attribute("cfg(test)");

    let expect = r#"
#[cfg(test)] 
mod foo {
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn scoped_imports() {
    let mut scope = Scope::new();
    scope
        .new_module("foo")
        .push_import("bar", "Bar")
        .push_import("bar", "baz::Baz")
        .push_import("bar::quux", "quuux::Quuuux")
        .new_struct("Foo")
        .push_named_field(Field::new("bar", "Bar"))
        .push_named_field(Field::new("baz", "baz::Baz"))
        .push_named_field(Field::new("quuuux", "quuux::Quuuux"));

    let expect = r#"
mod foo {
    use bar::{Bar, baz};
    use bar::quux::quuux;

    struct Foo {
        bar: Bar,
        baz: baz::Baz,
        quuuux: quuux::Quuuux,
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_in_mod() {
    let mut scope = Scope::new();

    {
        let module = scope.new_module("foo");
        module
            .new_struct("Foo")
            .set_doc("Hello some docs")
            .push_derive("Debug")
            .push_generic("T, U")
            .push_bound(Bound::new("T", ["SomeBound"]))
            .push_bound(Bound::new("U", ["SomeOtherBound"]))
            .push_named_field(Field::new("one", "T"))
            .push_named_field(Field::new("two", "U"));
    }

    let expect = r#"
mod foo {
    /// Hello some docs
    #[derive(Debug)]
    struct Foo<T, U>
    where T: SomeBound,
          U: SomeOtherBound,
    {
        one: T,
        two: U,
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}
