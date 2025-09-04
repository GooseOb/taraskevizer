#!/usr/bin/env node

import fs from 'fs';
import path from 'path';

// Define colors for directories
const colors = [
	'lightblue',
	'lightgreen',
	'lightyellow',
	'lightpink',
	'lightcyan',
	'orange',
	'gold',
	'violet',
	'salmon',
	'khaki',
];

// Ignore patterns (regex)
const ignorePatterns = [
	/node_modules/,
	/dist/,
	/build/,
	/\.test\.ts$/,
	/\.spec\.ts$/,
];

// Check if a file should be ignored
const shouldIgnore = (file) => ignorePatterns.some((pat) => pat.test(file));

// Recursively find all `.ts` files
function findTsFiles(dir) {
	let results = [];
	const entries = fs.readdirSync(dir, { withFileTypes: true });

	for (const entry of entries) {
		const fullPath = path.join(dir, entry.name);
		if (entry.isDirectory()) {
			if (!shouldIgnore(fullPath)) {
				results = results.concat(findTsFiles(fullPath));
			}
		} else if (entry.isFile() && fullPath.endsWith('.ts')) {
			if (!shouldIgnore(fullPath)) {
				results.push(path.relative(process.cwd(), fullPath));
			}
		}
	}
	return results;
}

const files = findTsFiles('.');

// Assign colors to directories
const dirColor = new Map();
const fileDir = new Map();
let colorIndex = 0;

for (const file of files) {
	const dir = path.dirname(file);
	if (!dirColor.has(dir)) {
		dirColor.set(dir, colors[colorIndex % colors.length]);
		colorIndex++;
	}
	fileDir.set(file, dir);
}

// Collect imports and exports
const imports = new Map();

for (const file of files) {
	const matches = [];
	if (!fs.existsSync(file)) continue;

	const lines = fs.readFileSync(file, 'utf8').split('\n');
	for (const line of lines) {
		const m = line.match(/(import|export).*['"](.+)['"];/);
		if (m) {
			const imported = m[2];
			const dir = path.dirname(file);
			const fullImport = path.resolve(dir, imported);

			let resolved = null;
			if (fs.existsSync(`${fullImport}.ts`)) {
				resolved = path.relative(process.cwd(), `${fullImport}.ts`);
			} else if (fs.existsSync(path.join(fullImport, 'index.ts'))) {
				resolved = path.relative(
					process.cwd(),
					path.join(fullImport, 'index.ts')
				);
			} else if (fs.existsSync(fullImport)) {
				resolved = path.relative(process.cwd(), fullImport);
			}

			if (resolved) matches.push(resolved);
		}
	}

	imports.set(file, matches.length > 0 ? matches : ['NONE']);
}

// Generate DOT graph
process.stdout.write('digraph G {');
process.stdout.write('  rankdir=LR;');
process.stdout.write('  node [shape=box, style=filled];');

// Nodes
for (const [file] of imports.entries()) {
	const dir = fileDir.get(file);
	const color = dirColor.get(dir);
	process.stdout.write(`  "${file}" [fillcolor=${color}];`);
}

// Edges
for (const [file, deps] of imports.entries()) {
	if (deps[0] !== 'NONE') {
		for (const imported of deps) {
			if (!shouldIgnore(imported)) {
				process.stdout.write(`  "${file}" -> "${imported}";`);
			}
		}
	}
}
process.stdout.write('}');
