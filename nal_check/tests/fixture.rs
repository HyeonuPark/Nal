extern crate nal_check;
extern crate serde_yaml;

use serde_yaml::from_str as yaml;

use nal_check::check;

macro_rules! fixture_ok {
    ($($test:expr)*) => ($(
        assert_eq!(
            check(&include_str!(concat!("fixtures_ok/", $test, ".nal"))
                .parse().expect(
                    concat!("Failed to parse ok/", $test, ".nal"))),
            Ok(()),
            concat!("Failed to check ok/", $test, ".nal")
        );
    )*);
}

macro_rules! fixture_err {
    ($($test:expr)*) => ($(
        assert_eq!(
            check(&include_str!(concat!("fixtures_err/", $test, ".nal"))
                .parse().expect(
                    concat!("Failed to parse err/", $test, ".nal"))),
            Err(yaml(include_str!(concat!("fixtures_err/", $test, ".yml")))
                .expect(concat!("Failed to parse ", $test, ".yml"))),
            concat!("Failed to check err/", $test, ".nal")
        );
    )*);
}

#[test]
fn test_ok() {
    fixture_ok!(
        "empty"
        "simple"
        "subscope"
    );
}

#[test]
fn test_err() {
    fixture_err!(
        "simple"
        "order"
    );
}
