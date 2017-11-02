use serde_yaml::from_str as yaml;
use super::parse;

macro_rules! compare_fixtures {
    ($($test:expr)*) => ($(
        assert_eq!(
            parse(include_str!(concat!("fixtures/", $test, ".nal")))
                .expect(concat!("Failed to parse ", $test, ".nal")),
            yaml(include_str!(concat!("fixtures/", $test, ".yml")))
                .expect(concat!("Failed to parse ", $test, ".yml")),
            concat!("\n\nFailed: ", $test, ", nal != yml\n\n")
        );
    )*);
}

#[test]
fn test_parser() {
    compare_fixtures!(
        "empty"
        "atom"
        "binary"
        "variable"
        "branch"
        "func"
        "string"
    );
}
