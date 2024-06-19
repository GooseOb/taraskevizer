#!/usr/bin/env node
import { readdir, readFile, writeFile } from 'fs/promises';
import * as path from 'path';

const files = await readdir('dist', { recursive: true, withFileTypes: true });

Promise.all(
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
