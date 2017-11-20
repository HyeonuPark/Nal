use serde_yaml::from_str as yaml;
use parse::parse;
use super::check;

macro_rules! fixture_ok {
    ($($name:ident, $test:expr)*) => ($(
        #[test]
        fn $name() {
            assert_eq!(
                check(&parse(include_str!(concat!("fixtures/ok/", $test, ".nal")))
                    .expect(concat!("Failed to parse ok/", $test, ".nal"))),
                Ok(()),
                concat!("\n\nFailed to check ok/", $test, ".nal\n\n")
            );
        }
    )*);
}

macro_rules! fixture_err {
    ($($name:ident, $test:expr)*) => ($(
        #[test]
        fn $name() {
            assert_eq!(
                check(&parse(include_str!(concat!("fixtures/err/", $test, ".nal")))
                    .expect(concat!("Failed to parse err/", $test, ".nal"))),
                Err(yaml(include_str!(concat!("fixtures/err/", $test, ".yml")))
                    .expect(concat!("Failed to parse ", $test, ".yml"))),
                concat!("\n\nFailed err/", $test, ", nal != yml\n\n")
            );
        }
    )*);
}

fixture_ok!(
    ok_empty, "empty"
    ok_simple, "simple"
    ok_subscope, "subscope"
);

fixture_err!(
    err_ident, "ident"
    err_object, "object"
);
