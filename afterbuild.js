#!/usr/bin/env node
import { readdir, readFile, writeFile } from 'fs/promises';
import { version } from './package.json';
import * as path from 'path';

const files = await readdir('dist', { recursive: true, withFileTypes: true });
const cliHelp = await readFile('cli-help.txt', 'utf8');

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
						.replace(/((?:im|ex)port.+from.+)(?<!\.js)(?=["'];)/g, '$1.js')
				)
			);
		})
);
await readFile('dist/bin.js', 'utf8').then((content) =>
	writeFile(
		'dist/bin.js',
		content
			.replace(/__VERSION__/g, `"${version}"`)
			.replace(/__CLI_HELP__/g, `\`${cliHelp}\``)
	)
);
