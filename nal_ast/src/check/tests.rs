use std::iter::empty as z;

use serde_yaml::from_str as yaml;
use parse::parse;
use super::check;

macro_rules! fixture_ok {
    ($($test:expr)*) => ($(
        assert_eq!(
            check(&parse(include_str!(concat!("fixtures/ok/", $test, ".nal")))
                .expect(concat!("Failed to parse ok/", $test, ".nal")), z()),
            Ok(()),
            concat!("\n\nFailed to check ok/", $test, ".nal\n\n")
        );
    )*);
}

macro_rules! fixture_err {
    ($($test:expr)*) => ($(
        assert_eq!(
            check(&parse(include_str!(concat!("fixtures/err/", $test, ".nal")))
                .expect(concat!("Failed to parse err/", $test, ".nal")), z()),
            Err(yaml(include_str!(concat!("fixtures/err/", $test, ".yml")))
                .expect(concat!("Failed to parse ", $test, ".yml"))),
            concat!("\n\nFailed err/", $test, ", nal != yml\n\n")
        );
    )*);
}

#[test]
fn test_checker_ok() {
    fixture_ok!(
        "empty"
        "simple"
        "subscope"
    );
}

#[test]
fn test_checker_err() {
    fixture_err!(
        "simple"
        "order"
        "ident"
    );
}
