#[test]
fn trybuild_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/compile_pass.rs");
    t.compile_fail("tests/trybuild/compile_fail.rs");
}
