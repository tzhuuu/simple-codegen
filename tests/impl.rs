use simple_codegen::*;

#[test]
fn empty_impl() {
    let mut scope = Scope::new();
    scope.new_impl("MyStruct");

    let expect = r#"
impl MyStruct {
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn impl_with_generics() {
    let mut scope = Scope::new();
    scope
        .new_impl(Type::new("MyStruct").with_generic("T"))
        .push_generic("T")
        .push_bound(Bound::new("T", ["Clone"]));

    let expect = r#"
impl<T> MyStruct<T>
where T: Clone,
{
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn impl_with_trait() {
    let mut scope = Scope::new();
    scope
        .new_impl("MyStruct")
        .set_impl_trait(Type::new("From").with_generic("String"));

    let expect = r#"
impl From<String> for MyStruct {
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn impl_with_associated_consts() {
    let mut scope = Scope::new();
    scope
        .new_impl("MyStruct")
        .push_associated_const(AssociatedConst::new("MY_CONST", "usize").with_concrete_value("0"));
    let expect = r#"
impl MyStruct {
    const MY_CONST: usize = 0;
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn impl_with_associated_types() {
    let mut scope = Scope::new();
    scope.new_impl("MyStruct").push_associated_type(
        AssociatedType::new("MY_TYPE").with_concrete_ty("usize", Vec::<String>::new()),
    );
    let expect = r#"
impl MyStruct {
    type MY_TYPE = usize;
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn impl_with_bounds() {
    let mut scope = Scope::new();
    scope
        .new_impl("MyStruct")
        .push_bound(Bound::new("T", ["SomeTrait", "AnotherTrait"]));
    let expect = r#"
impl MyStruct
where T: SomeTrait + AnotherTrait,
{
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn impl_with_macros() {
    let mut scope = Scope::new();
    scope
        .new_impl("MyStruct")
        .push_macro("#[async_trait::async_trait]");

    let expect = r#"
#[async_trait::async_trait]
impl MyStruct {
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn impl_with_single_function() {
    let mut scope = Scope::new();
    scope.new_impl("MyStruct").push_function(
        Function::new("my_function")
            .with_ret("usize")
            .with_line("0"),
    );

    let expect = r#"
impl MyStruct {
    fn my_function() -> usize {
        0
    }
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn impl_with_two_functions() {
    let mut scope = Scope::new();
    scope
        .new_impl("MyStruct")
        .push_function(
            Function::new("my_function")
                .with_ret("usize")
                .with_line("0"),
        )
        .push_function(
            Function::new("my_function2")
                .with_ret("&'static str")
                .with_line("\"test\""),
        );

    let expect = r#"
impl MyStruct {
    fn my_function() -> usize {
        0
    }

    fn my_function2() -> &'static str {
        "test"
    }
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}
