#!/usr/bin/env node
import { readdir, readFile, writeFile } from 'fs/promises';
import { lstatSync } from 'fs';
import * as path from 'path';

const files = await readdir('dist', {
	recursive: true,
	withFileTypes: true,
});

await Promise.all(
	files
		.filter((file) => file.isFile() && file.name.endsWith('.js'))
		.map((file) => {
			const fileName = path.resolve(file.parentPath, file.name);
			return readFile(fileName, 'utf8').then((content) =>
				writeFile(
					fileName,
					content
						.replace(/\/\*.*?\*\//gs, '')
						.replace(
							/((?:im|ex)port.+from\s+["'])(.+)(?<!\.js)(?=["'];)/g,
							(_$0, $1, $2) => {
								try {
									if (
										lstatSync(path.resolve(file.parentPath, $2)).isDirectory()
									) {
										$2 += '/index';
									}
								} catch {}
								return $1 + $2 + '.js';
							}
						)
				)
			);
		})
);

await Promise.all([
	readFile('dist/bin.js', 'utf8'),
	readFile('./package.json'),
	readFile('cli-help.txt', 'utf8'),
]).then(([content, pjson, cliHelp]) =>
	writeFile(
		'dist/bin.js',
		content
			.replace(/__VERSION__/g, `"${JSON.parse(pjson).version}"`)
			.replace(/__CLI_HELP__/g, `\`${cliHelp}\``)
	)
);
