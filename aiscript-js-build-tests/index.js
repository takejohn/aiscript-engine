// @ts-check

import { Parser } from '@syuilo/aiscript';
import { AiScriptError } from '@syuilo/aiscript/error.js';
import dedent from 'dedent';

import fs from 'fs/promises';
import path from 'path';

const testDir = path.resolve(import.meta.dirname, './tests/');
const resourceDir = path.resolve(testDir, './resources/');
const testFile = path.resolve(testDir, './parser_auto.rs');

await fs.mkdir(testDir, { recursive: true });
await fs.mkdir(resourceDir, { recursive: true });

await fs.writeFile(
    testFile,
    dedent`
        //! .isファイルから自動生成されたパーサのテスト

        fn test(script: &str, expected_ast_json: &str) {
            let script = aiscript_engine::string::Utf16String::from(script);
            let ast = aiscript_engine::Parser::new().parse(&script).unwrap();
            let expected_ast = serde_json::from_str::<Vec<aiscript_engine::ast::Node>>(expected_ast_json);
            match expected_ast {
                Ok(expected_ast) => {
                    pretty_assertions::assert_eq!(ast, expected_ast);
                },
                Err(_) => {
                    let ast = serde_json::to_value(ast).unwrap();
                    let expected_ast = serde_json::from_str::<serde_json::Value>(expected_ast_json).unwrap();
                    pretty_assertions::assert_eq!(ast, expected_ast);
                },
            }
        }
    ` + '\n'
);

const files = await fs.readdir(resourceDir);
await Promise.all(files.map(async file => {
    const extname = path.extname(file);
    if (extname != '.is') {
        return;
    }

    const script = await fs.readFile(path.resolve(resourceDir, file), 'utf-8');
    const astJson = JSON.stringify(parse(file, script), (_key, value) => {
        if (value instanceof Map) {
            return Object.fromEntries(value);
        }
        return value;
    });
    const basename = path.basename(file, '.is');
    const astJsonFilename = path.resolve(resourceDir, 'ast.' + basename + '.json');
    await fs.writeFile(astJsonFilename, astJson, 'utf-8');

    await fs.appendFile(
        testFile,
        dedent`
            #[test]
            fn test_${basename}() {
                test(include_str!("./resources/${basename}.is"), include_str!("./resources/ast.${basename}.json"));
            }
        ` + '\n'
    );
}));

/**
 * @param {string} filename
 * @param {string} script
 * @returns {import('@syuilo/aiscript').Ast.Node[]}
 */
function parse(filename, script) {
    try {
        return Parser.parse(script);
    } catch (e) {
        if (e instanceof AiScriptError) {
            throw new TypeError(`error while parsing ${filename}: ${e.message}`);
        } else {
            throw e;
        }
    }
}
