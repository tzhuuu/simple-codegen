use simple_codegen::*;

#[test]
fn type_alias_basic() {
    let mut scope = Scope::new();

    scope.new_type_alias("hello", "world").set_vis(Vis::Pub);

    let expect = r#"pub type hello = world;"#;

    assert_eq!(scope.to_string(), expect);
}
