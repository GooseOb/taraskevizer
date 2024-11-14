import {
	createProgram,
	getPreEmitDiagnostics,
	flattenDiagnosticMessageText,
	readConfigFile,
	parseJsonConfigFileContent,
	sys,
} from 'typescript';
import { readdirSync, lstatSync } from 'fs';
import { readFile, writeFile } from 'fs/promises';
import { join, resolve, relative } from 'path';
import { fileURLToPath } from 'url';

const getPrinter =
	<T extends { write: (s: string) => void }>(writer: T, clr = '33') =>
	(msg: string) =>
		writer.write(`\x1b[${clr}m[build]\x1b[0m ${msg}\n`);
const printErr = getPrinter(process.stderr, '31');
const print = getPrinter(process.stdout);
const printWithTime = (msg: string) => {
	const time = performance.now();
	process.stdout.write(
		`\x1b[33m[${(time - lastTime).toFixed(2)}ms]\x1b[0m ${msg}\n`
	);
	lastTime = time;
};

print('Starting build process...');

const startTime = performance.now();
let lastTime = startTime;
let exitCode = 0;

const __dirname = fileURLToPath(new URL('.', import.meta.url));

const parseConfig = (dir: string, file = 'tsconfig.json') => {
	const absoluteDir = join(__dirname, dir);
	return parseJsonConfigFileContent(
		readConfigFile(join(absoluteDir, file), sys.readFile).config,
		sys,
		absoluteDir
	).options;
};

const compilerOptions = parseConfig('src');

printWithTime('Compiler options parsed');

const srcFiles = readdirSync('src', {
	recursive: true,
	withFileTypes: true,
}).filter((file) => file.isFile() && file.name.endsWith('.ts'));

printWithTime('Source file paths read');

const program = createProgram(
	srcFiles.map(({ parentPath, name }) => join(parentPath, name)),
	compilerOptions
);

printWithTime('Program created');

const emitResult = program.emit();

if (emitResult.emitSkipped) {
	printErr('File emitting was skipped');
	exitCode = 1;
} else {
	printWithTime('Files emited');
}

const allDiagnostics = getPreEmitDiagnostics(program).concat(
	emitResult.diagnostics
);
for (const diagnostic of allDiagnostics) {
	if (diagnostic.file) {
		const { line, character } = diagnostic.file.getLineAndCharacterOfPosition(
			diagnostic.start!
		);
		const message = flattenDiagnosticMessageText(diagnostic.messageText, '\n');
		print(
			`${diagnostic.file.fileName} (${line + 1},${character + 1}): ${message}`
		);
	} else {
		console.log(flattenDiagnosticMessageText(diagnostic.messageText, '\n'));
	}
}

printWithTime('Diagnostics printed');

const distPath = join(__dirname, 'dist');

await Promise.all(
	srcFiles.map((file) => {
		const parentPath = join(__dirname, file.parentPath.replace(/^src/, 'dist'));
		const filePath = join(parentPath, file.name.replace(/ts$/, 'js'));
		return readFile(filePath, 'utf8').then((content) =>
			writeFile(
				filePath,
				content
					.replace(/\/\*.*?\*\//gs, '')
					.replace(
						/((?:im|ex)port.+from\s+["'])@\/(.+)(?=["'];)/g,
						(_$0, $1, $2) => $1 + relative(parentPath, join(distPath, $2))
					)
					.replace(
						/((?:im|ex)port.+from\s+["'])(.+)(?<!\.js)(?=["'];)/g,
						(_$0, $1, $2) => {
							try {
								if (lstatSync(resolve(parentPath, $2)).isDirectory()) {
									$2 += '/index';
								}
							} catch {
								// ignore errors
							}
							return $1 + $2 + '.js';
						}
					)
			)
		);
	})
);

printWithTime('Imports fixed');

const CLI_PREFIX = `\x1b[34m[taraskevizer]\x1b[0m`;

const colorizeText = (text: string) =>
	text
		.replace(/\{\\fix ([^}]+)}/g, '\x1b[32m$1\x1b[0m')
		.replace(/\n([^:\n]+):\n/g, '\n\x1b[33m$1\x1b[0m:\n')
		.replace(/(\s)tarask(?=\s)/g, '$1\x1b[34mtarask\x1b[0m')
		.replace(/(\s)(--?\S+)/g, '$1\x1b[35m$2\x1b[0m')
		.replace(/\{\\PREFIX}/g, CLI_PREFIX);

await Promise.all([
	readFile('./dist/bin.js', 'utf8'),
	readFile('./package.json', 'utf8'),
	readFile('./cli-help.txt', 'utf8'),
]).then(([content, pjson, cliHelp]) =>
	writeFile(
		'dist/bin.js',
		content
			.replace(/__VERSION__/g, `"${JSON.parse(pjson).version}"`)
			.replace(/__CLI_HELP__/g, `\`${colorizeText(cliHelp)}\``)
			.replace(/__CLI_PREFIX__/g, `"${CLI_PREFIX}"`)
	)
);

printWithTime('Version and help text injected');

print(`Done in ${(performance.now() - startTime).toFixed(2)}ms`);
process.exit(exitCode);
