extern crate nal_parser as parser;
extern crate serde_yaml as yaml;

macro_rules! compare_fixtures {
    ($($test:expr)*) => ($(
        assert_eq!(
            parser::parse(include_str!(
                concat!("fixtures/", $test, ".nal")))
                    .expect(concat!("Failed to parse ", $test, ".nal")),
            yaml::from_str(include_str!(
                concat!("fixtures/", $test, ".yml")))
                    .expect(concat!("Failed to parse ", $test, ".yml")),
            concat!("\n\nFailed to test fixture: ", $test, "\n\n")
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
    );
}
