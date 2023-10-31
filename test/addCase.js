import { appendFile, readFile, writeFile } from 'fs/promises';
import { fileURLToPath } from 'url';
import path from 'path';

const fileNameWithoutExt = process.argv[2];

if (!fileNameWithoutExt) {
	process.stderr.write(
		'Use \x1b[35maddCase.js \x1b[33m<fileName> <varName?>\x1b[0m\n'
	);
	process.exit(1);
}

const varName =
	process.argv[3] ||
	fileNameWithoutExt.replace(/-(.)/g, ($0, $1) => $1.toUpperCase());
const testName = varName[0].toUpperCase() + varName.slice(1);

const testPath = path.dirname(fileURLToPath(import.meta.url));
const casesPath = path.resolve(testPath, 'cases');
const casesIndexPath = path.resolve(casesPath, 'index.ts');
const filePath = path.resolve(casesPath, fileNameWithoutExt + '.ts');
const indexTestPath = path.resolve(testPath, 'index.ts');
const bunIndexTestPath = path.resolve(testPath, 'bun', 'index.test.ts');

const addToFile = (filePath, code) =>
	readFile(filePath, 'utf8').then((content) =>
		writeFile(
			filePath,
			content.replace(/\n(?=\/\/ add a new case here)/, `\n${code}\n\n`)
		)
	);

await writeFile(filePath, "export default [['', '']] as const;\n");

await appendFile(
	casesIndexPath,
	`export { default as ${varName} } from './${fileNameWithoutExt}';\n`
);

await addToFile(
	indexTestPath,
	`test('${testName}', tarask, cases.${varName});`
);

await addToFile(
	bunIndexTestPath,
	`testOnCases('\\x1b[31m${testName}', tarask, cases.${varName});`
);
