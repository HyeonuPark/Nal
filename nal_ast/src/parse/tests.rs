use serde_yaml::from_str as yaml;
use super::parse;

macro_rules! compare_fixtures {
    ($($name:ident, $test:expr)*) => ($(
        #[test]
        fn $name() {
            assert_eq!(
                parse(include_str!(concat!("fixtures/", $test, ".nal")))
                    .expect(concat!("Failed to parse ", $test, ".nal")),
                yaml(include_str!(concat!("fixtures/", $test, ".yml")))
                    .expect(concat!("Failed to parse ", $test, ".yml")),
                concat!("\n\nFailed: ", $test, ", nal != yml\n\n")
            );
        }
    )*);
}

compare_fixtures!(
    empty, "empty"
    atom, "atom"
    binary, "binary"
    variable, "variable"
    branch, "branch"
    func, "func"
    string, "string"
);
