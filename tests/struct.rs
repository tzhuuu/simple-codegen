use simple_codegen::*;

#[test]
fn empty_struct() {
    let mut scope = Scope::new();
    scope.new_struct("MyStruct");

    let expect = "struct MyStruct;";

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn struct_basic() {
    let mut scope = Scope::new();
    scope
        .new_struct("MyStruct")
        .set_vis(Vis::Pub)
        .push_named_field(Field::new("foo", "usize"))
        .push_named_field(Field::new("bar", "String"))
        .set_doc("This is a test struct.")
        .push_lint(Lint::allow("clippy::struct_excessive_bools"))
        .push_lint(Lint::allow("clippy::needless_bools"))
        .push_derive("Clone")
        .push_derive("Debug")
        .push_derive("Serialize")
        .push_derive("Deserialize")
        .push_attribute("serde(rename_all = \"snake_case\")")
        .push_generic("T")
        .push_bound(Bound::new("T", ["Clone"]));

    let expect = r#"
/// This is a test struct.
#[allow(clippy::struct_excessive_bools)]
#[allow(clippy::needless_bools)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MyStruct<T>
where T: Clone,
{
    foo: usize,
    bar: String,
}"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn struct_with_tuple_fields() {
    let mut scope = Scope::new();
    scope
        .new_struct("MyStruct")
        .set_vis(Vis::Pub)
        .push_tuple_field("usize")
        .push_tuple_field("String");

    let expect = r#"
pub struct MyStruct(usize, String);"#;

    assert_eq!(scope.to_string(), expect.trim_start());
}

#[test]
fn struct_with_repr() {
    let mut scope = Scope::new();

    scope
        .new_struct("MyStruct")
        .set_repr(Some(String::from("C")))
        .push_named_field(Field::new("one", "u8"))
        .push_named_field(Field::new("two", "u8"));

    let expect = r#"
#[repr(C)]
struct MyStruct {
    one: u8,
    two: u8,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_generics_1() {
    let mut scope = Scope::new();

    scope
        .new_struct("MyStruct")
        .push_generic("T")
        .push_generic("U")
        .push_named_field(Field::new("one", "T"))
        .push_named_field(Field::new("two", "U"));

    let expect = r#"
struct MyStruct<T, U> {
    one: T,
    two: U,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_generics_2() {
    // Note that we allow setting multiple generics in a single string.
    let mut scope = Scope::new();

    scope
        .new_struct("MyStruct")
        .push_generic("T, U")
        .push_named_field(Field::new("one", "T"))
        .push_named_field(Field::new("two", "U"));

    let expect = r#"
struct MyStruct<T, U> {
    one: T,
    two: U,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_generics_3() {
    let mut scope = Scope::new();

    scope
        .new_struct("MyStruct")
        .push_generic("T: Win, U")
        .push_named_field(Field::new("one", "T"))
        .push_named_field(Field::new("two", "U"));

    let expect = r#"
struct MyStruct<T: Win, U> {
    one: T,
    two: U,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_where_clause_1() {
    let mut scope = Scope::new();

    scope
        .new_struct("MyStruct")
        .push_generic("T")
        .push_bound(Bound::new("T", ["Foo"]))
        .push_named_field(Field::new("one", "T"));

    let expect = r#"
struct MyStruct<T>
where T: Foo,
{
    one: T,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_where_clause_2() {
    let mut scope = Scope::new();

    scope
        .new_struct("MyStruct")
        .push_generic("T, U")
        .push_bound(Bound::new("T", ["Foo"]))
        .push_bound(Bound::new("U", ["Baz"]))
        .push_named_field(Field::new("one", "T"))
        .push_named_field(Field::new("two", "U"));

    let expect = r#"
struct MyStruct<T, U>
where T: Foo,
      U: Baz,
{
    one: T,
    two: U,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_member_visibility() {
    let mut scope = Scope::new();

    let struct_description = scope.new_struct("MyStruct");

    let mut bar = Field::new("field1", "usize");
    bar.set_vis(Vis::Pub);

    struct_description.push_named_field(bar);
    struct_description.push_named_field(Field::new("field2", "i16").with_vis(Vis::PubCrate));

    let expect = r#"
struct MyStruct {
    pub field1: usize,
    pub(crate) field2: i16,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn single_struct_documented_field() {
    let mut scope = Scope::new();

    let doc = "Field's documentation\nSecond line";

    let mut struct_ = Struct::new("Foo");

    let mut field1 = Field::new("one", "usize");
    field1.set_doc(doc);
    struct_.push_named_field(field1);

    let mut field2 = Field::new("two", "usize");
    field2.push_annotation("#[serde(rename = \"bar\")]");
    struct_.push_named_field(field2);

    let mut field3 = Field::new("three", "usize");
    field3
        .set_doc(doc)
        .push_annotation("#[serde(skip_serializing)]\n#[serde(skip_deserializing)]");
    struct_.push_named_field(field3);

    scope.push_struct(struct_);

    let expect = r#"
struct Foo {
    /// Field's documentation
    /// Second line
    one: usize,
    #[serde(rename = "bar")]
    two: usize,
    /// Field's documentation
    /// Second line
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    three: usize,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}
