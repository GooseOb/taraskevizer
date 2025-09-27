import {
	createProgram,
	getPreEmitDiagnostics,
	flattenDiagnosticMessageText,
	readConfigFile,
	parseJsonConfigFileContent,
	sys,
} from 'typescript';
import { lstat, readFile, writeFile, readdir } from 'node:fs/promises';
import { join, resolve, relative } from 'node:path';
import { build } from 'esbuild';

const getPrinter =
	(writer: { write: (s: string) => void }, clr = '33') =>
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

const parseConfig = (dir: string, file = 'tsconfig.json') => {
	const absoluteDir = resolve(dir);
	return parseJsonConfigFileContent(
		readConfigFile(join(absoluteDir, file), sys.readFile).config,
		sys,
		absoluteDir
	).options;
};

const compilerOptions = parseConfig('src');

printWithTime('Compiler options parsed');

const srcFiles = (
	await readdir('src', {
		recursive: true,
		withFileTypes: true,
	})
).filter((file) => file.isFile() && file.name.endsWith('.ts'));

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

const DIST_PATH = resolve('dist');

const replaceAsync = (
	str: string,
	regex: RegExp,
	replacer: (...args: string[]) => Promise<string>
) => {
	const promises: Promise<string>[] = [];
	str.replace(regex, (...args: string[]) => {
		promises.push(replacer(...args));
		return '';
	});
	return Promise.all(promises).then((data) => {
		let i = 0;
		return str.replace(regex, () => data[i++]);
	});
};

type MaybePromise<T> = T | Promise<T>;

const transformFile = (
	filePath: string,
	cb: (content: string) => MaybePromise<string>
) =>
	readFile(filePath, 'utf8')
		.then(cb)
		.then((content) => writeFile(filePath, content));

const getResolveAt = (parentPath: string) => (code: string) =>
	code.replace(
		/((?:im|ex)port.+from\s+["'])@\/(.+)(?=["'];)/g,
		(_$0, $1, $2) => $1 + relative(parentPath, join(DIST_PATH, $2))
	);

await Promise.all(
	srcFiles.map((file) => {
		const parentPath = resolve(file.parentPath.replace(/^src/, 'dist'));
		const resolveAt = getResolveAt(parentPath);
		return Promise.all([
			transformFile(join(parentPath, file.name.replace(/ts$/, 'js')), (code) =>
				replaceAsync(
					resolveAt(code.replace(/\/\*.*?\*\//gs, '')),
					/((?:im|ex)port.+from\s+["'])(.+)(?<!\.js)(?=["'];)/g,
					(_$0, $1, $2) =>
						lstat(resolve(parentPath, $2))
							.then(
								(stat) => stat.isDirectory(),
								() => false
							)
							.then((isDir) => $1 + $2 + (isDir ? '/index.js' : '.js'))
				)
			),
			transformFile(
				join(parentPath, file.name.replace(/ts$/, 'd.ts')),
				resolveAt
			),
		]);
	})
);

printWithTime('Imports fixed');

const CLI_PREFIX = '\x1b[34m[taraskevizer]\x1b[0m';

const colorizeText = (text: string) =>
	text
		.replace(/\{\\fix ([^}]+)}/g, '\x1b[32m$1\x1b[0m')
		.replace(/\n([^:\n]+):\n/g, '\n\x1b[33m$1\x1b[0m:\n')
		.replace(/(\s)tarask(?=\s)/g, '$1\x1b[34mtarask\x1b[0m')
		.replace(/(\s)(--?\S+)/g, '$1\x1b[35m$2\x1b[0m')
		.replace(/\{\\PREFIX}/g, CLI_PREFIX);

const BIN_PATH = resolve('dist', 'bin.js');

await Promise.all([
	readFile(BIN_PATH, 'utf8'),
	readFile('package.json', 'utf8'),
	readFile('cli-help.txt', 'utf8'),
]).then(([content, pjson, cliHelp]) =>
	writeFile(
		BIN_PATH,
		content
			.replace(/__VERSION__/g, `"${JSON.parse(pjson).version}"`)
			.replace(/__CLI_HELP__/g, `\`${colorizeText(cliHelp)}\``)
			.replace(/__CLI_PREFIX__/g, `"${CLI_PREFIX}"`)
	)
);

printWithTime('Version and help text injected');

await build({
	entryPoints: [resolve(DIST_PATH, 'index.js')],
	bundle: true,
	outfile: resolve(DIST_PATH, 'bundle.js'),
	charset: 'utf8',
	globalName: 'taraskevizer',
	minify: true,
	sourcemap: false,
});

printWithTime('Bundled with esbuild');

print(`Done in ${(performance.now() - startTime).toFixed(2)}ms`);
process.exit(exitCode);
