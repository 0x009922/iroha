#[test]
pub fn pass() {
    macrotest::expand("tests/expand/*.rs");
    // // Alternatively,
    // macrotest::expand_without_refresh("tests/expand/*.rs");
}
