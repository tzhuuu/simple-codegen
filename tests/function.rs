use simple_codegen::*;

#[test]
fn function_basic() {
    let mut scope = Scope::new();
    scope
        .new_function("test_fn")
        .set_vis(Vis::Pub)
        .push_arg("foo", "uint")
        .set_ret("uint")
        .push_line("let res = foo + 1;")
        .push_line("res")
        .set_doc("This is a test function.");

    let expect = r#"
/// This is a test function.
pub fn test_fn(foo: uint) -> uint {
    let res = foo + 1;
    res
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
#[should_panic(expected = "impl blocks must define fn bodies")]
fn function_without_body() {
    let mut scope = Scope::new();
    scope.new_function("test_fn");
    scope.to_string();
}

#[test]
fn function_with_lint() {
    let mut scope = Scope::new();
    scope
        .new_function("test_fn")
        .set_ret("uint")
        .push_line("1")
        .push_lint(Lint::allow("clippy::too_many_arguments"));

    let expect = r#"
#[allow(clippy::too_many_arguments)]
fn test_fn() -> uint {
    1
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn function_with_generics_and_bounds() {
    let mut scope = Scope::new();
    scope
        .new_function("test_fn")
        .set_async(true)
        .push_generic("T")
        .push_bound(Bound::new("T", ["Clone"]))
        .set_ret("T")
        .push_line("unimplemented!();");

    let expect = r#"
async fn test_fn<T>() -> T
where T: Clone,
{
    unimplemented!();
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}
