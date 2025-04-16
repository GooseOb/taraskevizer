import { appendFile, readFile, writeFile, exists } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

if (process.argv.length < 3) {
	process.stderr.write(
		'Use \x1b[35maddCase.js \x1b[33m<fileName> [<varName>]\x1b[0m\n'
	);
	process.exit(1);
}

const fileNameWithoutExt = process.argv[2];

const testPath = path.dirname(fileURLToPath(import.meta.url));
const casesPath = path.resolve(testPath, 'cases');
const casesIndexPath = path.resolve(casesPath, 'index.ts');
const filePath = path.resolve(casesPath, fileNameWithoutExt + '.ts');
const indexTestPath = path.resolve(testPath, 'index.ts');
const bunIndexTestPath = path.resolve(testPath, 'bun', 'index.test.ts');

if (await exists(filePath)) {
	process.stderr.write(`\x1b[35m${fileNameWithoutExt}\x1b[0m already exists\n`);
	process.exit(1);
}

const varName =
	process.argv[3] ||
	fileNameWithoutExt.replace(/-(.)/g, ($0, $1) => $1.toUpperCase());
const testName = varName[0].toUpperCase() + varName.slice(1);

const addToFile = (filePath, code) =>
	readFile(filePath, 'utf8').then((content) =>
		writeFile(
			filePath,
			content.replace(/\n(?=\/\/ add a new case here)/, `\n${code}\n\n`)
		)
	);

await Promise.all([
	writeFile(filePath, "export default [['', '']] as const;\n"),

	appendFile(
		casesIndexPath,
		`export { default as ${varName} } from './${fileNameWithoutExt}';\n`
	),

	addToFile(
		indexTestPath,
		`test('${testName}', (text) => tarask(text), cases.${varName});`
	),

	addToFile(
		bunIndexTestPath,
		`testOnCases('\\x1b[31m${testName}', (text) => tarask(text), cases.${varName});`
	),
]);
