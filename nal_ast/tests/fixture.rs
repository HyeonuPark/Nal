#[macro_use]
extern crate pretty_assertions;
extern crate nal_ast;
extern crate serde_yaml as yaml;

use nal_ast::parser;

macro_rules! compare_fixtures {
    ($($test:expr)*) => ($(
        assert_eq!(
            parser::parse(include_str!(
                concat!("fixtures/", $test, ".nal")))
                    .expect(concat!("Failed to parse ", $test, ".nal")),
            yaml::from_str(include_str!(
                concat!("fixtures/", $test, ".yml")))
                    .expect(concat!("Failed to parse ", $test, ".yml")),
            concat!("\n\nFailed: ", $test, ", nal != yml\n\n")
        );
    )*);
}

#[test]
fn test_fixtures() {
    compare_fixtures!(
        "empty"
        "atom"
        "binary"
        "variable"
        "branch"
    );
}
