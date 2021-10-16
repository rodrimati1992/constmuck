#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/main_tests/ui/*err.rs");
    t.pass("tests/main_tests/ui/*fine.rs");
}
