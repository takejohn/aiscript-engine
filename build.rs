use std::{fs::{self, File}, io::Write, path::Path, process::{Command, Stdio}};

use indoc::indoc;

const AISCRIPT_JS_BUILD_TESTS_PATH: &str = "./aiscript-js-build-tests/";

const RESOURCES_PATH: &str = "./tests/resources/";

const AISCRIPT_EXT: &str = "is";

const TEST_FILE_NAME: &str = "./tests/parser_auto.rs";

fn main() {
    build_parser_tests();
}

fn build_parser_tests() {
    Command::new("node")
        .arg(AISCRIPT_JS_BUILD_TESTS_PATH)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output().unwrap();

    let mut test_file = File::create(TEST_FILE_NAME).unwrap();
    write!(test_file, indoc! {r#"
        //! .isファイルから自動生成されたパーサのテスト

        fn test(script: &str, expected_ast_json: &str) {{
            let script = aiscript_engine::string::Utf16String::from(script);
            let ast = aiscript_engine::Parser::new().parse(&script).unwrap();
            let expected_ast = serde_json::from_str::<Vec<aiscript_engine::ast::Node>>(expected_ast_json);
            match expected_ast {{
                Ok(expected_ast) => {{
                    pretty_assertions::assert_eq!(ast, expected_ast);
                }},
                Err(_) => {{
                    let ast = serde_json::to_value(ast).unwrap();
                    let expected_ast = serde_json::from_str::<serde_json::Value>(expected_ast_json).unwrap();
                    pretty_assertions::assert_eq!(ast, expected_ast);
                }},
            }}
        }}
    "#}).unwrap();

    for item in fs::read_dir(RESOURCES_PATH).unwrap().into_iter() {
        let item = item.unwrap();
        let path = item.path();
        if !path.extension().is_some_and(|ext| ext.to_str().is_some_and(|ext| ext == AISCRIPT_EXT))  {
            continue;
        }
        let stem = Path::new(path.file_name().unwrap()).file_stem().unwrap().to_str().unwrap();
        write!(test_file, indoc! {r#"

            #[test]
            fn test_{stem}() {{
                test(include_str!("./resources/{stem}.is"), include_str!("./resources/{stem}.is.ast.json"));
            }}
        "#}, stem = stem).unwrap();
    }
}
