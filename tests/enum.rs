use simple_codegen::*;

#[test]
fn empty_enum() {
    let mut scope = Scope::new();
    scope.new_enum("MyEnum");

    let expect = r#"
enum MyEnum {
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn enum_basic() {
    let mut scope = Scope::new();
    scope
        .new_enum("MyEnum")
        .set_vis(Vis::Pub)
        .push_variant("VariantA")
        .push_variant("VariantB")
        .set_doc("This is a test enum.")
        .push_lint(Lint::allow("clippy::enum_variant_names"));

    let expect = r#"
/// This is a test enum.
#[allow(clippy::enum_variant_names)]
pub enum MyEnum {
    VariantA,
    VariantB,
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn enum_with_complex_variants() {
    let mut scope = Scope::new();
    scope
        .new_enum("MyEnum")
        .push_derive("Debug")
        .push_generic("T")
        .push_bound(Bound::new("T", ["Clone"]))
        .push_variant(Variant::new("VariantA").with_named_field("test", "String"))
        .push_variant(Variant::new("VariantB").with_tuple_field("usize"))
        .push_variant(Variant::new("VariantC").with_tuple_field("T"));

    let expect = r#"
#[derive(Debug)]
enum MyEnum<T>
where T: Clone,
{
    VariantA {
        test: String,
    }
    ,
    VariantB(usize),
    VariantC(T),
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn enum_with_repr() {
    let mut scope = Scope::new();

    scope
        .new_enum("IpAddrKind")
        .set_repr(Some(String::from("u8")))
        .push_variant(Variant::new("V4"))
        .push_variant(Variant::new("V6"));

    let expect = r#"
#[repr(u8)]
enum IpAddrKind {
    V4,
    V6,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn enum_with_multiple_allow() {
    let mut scope = Scope::new();

    scope
        .new_enum("IpAddrKind")
        .push_lint(Lint::allow("dead_code"))
        .push_lint(Lint::allow("clippy::all"))
        .push_variant(Variant::new("V4"))
        .push_variant(Variant::new("V6"));

    let expect = r#"
#[allow(dead_code)]
#[allow(clippy::all)]
enum IpAddrKind {
    V4,
    V6,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}
