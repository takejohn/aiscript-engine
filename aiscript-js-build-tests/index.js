// @ts-check

import { Parser } from '@syuilo/aiscript';
import { AiScriptError } from '@syuilo/aiscript/error.js';
import dedent from 'dedent';

import fs from 'fs/promises';
import path from 'path';

const INDENT_STRING = '    ';
const TEST_DIR = path.resolve(import.meta.dirname, './tests/');
const RESOURCE_DIR = path.resolve(TEST_DIR, './resources/');
const TEST_FILE = path.resolve(TEST_DIR, './parser_auto.rs');

await fs.mkdir(TEST_DIR, { recursive: true });
await fs.mkdir(RESOURCE_DIR, { recursive: true });

await fs.writeFile(
    TEST_FILE,
    dedent`
        //! .isファイルから自動生成されたパーサのテスト
        mod utils {
            pub(super) fn test(script: &str, expected_ast_json: &str) {
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
            pub(super) fn fails(script: &str) {
                let script = aiscript_engine::string::Utf16String::from(script);
                assert!(aiscript_engine::Parser::new().parse(&script).is_err());
            }
        }
        mod tests {
    ` + '\n'
);
await visitDir([]);
await write('}', 0);

/**
 * @param {string[]} pathArray
 */
async function visitDir(pathArray) {
    const dirname = path.resolve(RESOURCE_DIR, ...pathArray);
    const depth = pathArray.length + 1;
    const files = await fs.readdir(dirname, {
        withFileTypes: true
    });

    for (const entry of files) {
        if (entry.isDirectory()) {
            const name = entry.name;
            await write(`mod ${name} {`, depth);
            await visitDir([...pathArray, name]);
            await write(`}`, depth);
        }

        const filename = entry.name;
        const extname = path.extname(filename);
        if (extname != '.is') {
            continue;
        }

        const scriptFilename = path.resolve(dirname, filename);
        const script = await fs.readFile(scriptFilename, 'utf-8');
        const ast = parse(filename, script);
        const basename = path.basename(filename, '.is');

        if (!(ast instanceof AiScriptError)) {
            const astJson = JSON.stringify(ast, (_key, value) => {
                if (value instanceof Map) {
                    return Object.fromEntries(value);
                }
                return value;
            });
            const astJsonFilename = path.resolve(dirname, 'ast.' + basename + '.json');
            await fs.writeFile(astJsonFilename, astJson, 'utf-8');

            await write(`
                #[test]
                fn ${basename}() {
                    crate::utils::test(
                        include_str!("${path.relative(TEST_DIR, scriptFilename).split(path.sep).join('/')}"),
                        include_str!("${path.relative(TEST_DIR, astJsonFilename).split(path.sep).join('/')}")
                    );
                }
            `, depth);
        } else {
            await write(`
                #[test]
                fn ${basename}() {
                    crate::utils::fails(include_str!("${path.relative(TEST_DIR, scriptFilename).split(path.sep).join('/')}"));
                }
            `, depth)
        }
    }
}

/**
 * @param {string} s
 * @param {number} indent
 */
async function write(s, indent) {
    const formatted = dedent(s).split('\n').map(
        (line) => INDENT_STRING.repeat(indent) + line
    ).join('\n') + '\n';
    await fs.appendFile(TEST_FILE, formatted);
}

/**
 * @param {string} filename
 * @param {string} script
 * @returns {import('@syuilo/aiscript').Ast.Node[] | AiScriptError}
 */
function parse(filename, script) {
    try {
        return Parser.parse(script);
    } catch (e) {
        if (e instanceof AiScriptError) {
            return e;
        } else {
            throw e;
        }
    }
}
