use std::process::{Command, Stdio};

const AISCRIPT_JS_BUILD_TESTS_PATH: &str = "./aiscript-js-build-tests/";

fn main() {
    build_parser_tests();
}

fn build_parser_tests() {
    Command::new("node")
        .arg(AISCRIPT_JS_BUILD_TESTS_PATH)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
}
