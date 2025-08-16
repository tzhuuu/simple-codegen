use simple_codegen::*;

#[test]
fn empty_trait() {
    let mut scope = Scope::new();
    scope.new_trait("MyTrait");

    let expect = r#"
trait MyTrait {
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn trait_with_vis() {
    let mut scope = Scope::new();
    scope.new_trait("MyTrait").set_vis(Vis::Pub);

    let expect = r#"
pub trait MyTrait {
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn trait_with_attributes() {
    let mut scope = Scope::new();
    scope
        .new_trait("MyTrait")
        .push_attribute("cfg(feature = \"special\")");

    let expect = r#"
#[cfg(feature = "special")]
trait MyTrait {
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn trait_with_invalid_bounds() {
    // Note that this test illustrates that we do not enforce that a bound is from a valid generic.
    let mut scope = Scope::new();
    scope
        .new_trait("MyTrait")
        .push_generic("S")
        .push_bound(Bound::new("T", ["Clone"]));

    let expect = r#"
trait MyTrait<S>
where T: Clone,
{
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn trait_with_macros() {
    let mut scope = Scope::new();
    scope.new_trait("MyTrait").push_macro("#[async_trait]");

    let expect = r#"
#[async_trait]
trait MyTrait {
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn trait_with_parents() {
    let mut scope = Scope::new();
    scope
        .new_trait("MyTrait")
        .push_parent("ParentTrait1")
        .push_parent("ParentTrait2");

    let expect = r#"
trait MyTrait: ParentTrait1 + ParentTrait2 {
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn trait_with_doc() {
    let mut scope = Scope::new();
    scope
        .new_trait("MyTrait")
        .set_doc("This is a trait comment.");

    let expect = r#"    
/// This is a trait comment.
trait MyTrait {
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn trait_with_associated_consts() {
    let mut scope = Scope::new();
    scope
        .new_trait("MyTrait")
        .push_associated_const(AssociatedConst::new("MY_CONST", "i32"));

    let expect = r#"
trait MyTrait {
    const MY_CONST: i32;
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn trait_with_associated_types() {
    let mut scope = Scope::new();
    scope
        .new_trait("MyTrait")
        .push_associated_type(AssociatedType::new_with_bounds("Item", ["Copy"]));

    let expect = r#"
trait MyTrait {
    type Item: Copy;
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn trait_with_functions() {
    let mut scope = Scope::new();
    scope.new_trait("MyTrait").push_function(
        Function::new("my_function")
            .with_arg("arg1", Type::new("i32"))
            .with_ret(Type::new("String")),
    );

    let expect = r#"
trait MyTrait {
    fn my_function(arg1: i32) -> String;
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}
