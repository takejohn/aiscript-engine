import { Parser } from "@syuilo/aiscript";

import fs from "fs/promises";
import path from "path";

const dirname = path.resolve(import.meta.dirname, "../tests/resources/");

const files = await fs.readdir(dirname);

await Promise.all(files.map(async file => {
    const extname = path.extname(file);
    if (extname != '.is') {
        return;
    }

    const script = await fs.readFile(path.resolve(dirname, file), 'utf-8');
    const astJson = JSON.stringify(Parser.parse(script));
    const astJsonFilename = path.resolve(dirname, path.basename(file) + '.ast.json');
    await fs.writeFile(astJsonFilename, astJson, 'utf-8');
}));
