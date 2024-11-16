#[test]
fn test_get_go_program() {
    let result = rust_lib::get_go_program();
    assert!(result.ends_with("go_program_cross"), "{}", result.display());
}
