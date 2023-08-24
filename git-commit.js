import { readFile, writeFile, unlink } from 'node:fs/promises';
import simpleGit from 'simple-git';
import path from 'path';

const prefix = '\x1b[35m[git-commit]\x1b[0m';
const print = (...msgs) => {
	console.log(prefix, ...msgs);
};

if (process.argv.length < 3) {
	print('No arguments passed');
	process.exit(1);
}

let msg;
let commitOptions = process.argv.slice(2);

const removeRange = (arr, from, to) =>
	arr.slice(0, from).concat(commitOptions.slice(to));

for (let i = 0; i < commitOptions.length; i++) {
	const item = commitOptions[i];
	if (item === '-m' || item === '-am') {
		msg = commitOptions[i + 1];
		if (item === '-m') {
			commitOptions = removeRange(commitOptions, i, i + 2);
		} else {
			commitOptions[i] = '-a';
			commitOptions = removeRange(commitOptions, i + 1, i + 2);
		}
		break;
	}
}

if (!msg) {
	print('Commit message is required');
	process.exit(1);
}

const is = {
	committing: false,
	inputProcessing: true,
};
const printCommitSummary = (branch, msg, { changes, insertions, deletions }) =>
	print(
		`${branch}: ${msg}
changed files: \x1b[33m${changes}\x1b[0m insertions: \x1b[32m${insertions}\x1b[0m deletions: \x1b[31m${deletions}\x1b[0m
package version: ${pkg.version}`
	);

const TEMP_FILE_PATH = path.resolve('.git', 'commit-safe');
const commit = async () => {
	is.committing = true;
	await writeFile(TEMP_FILE_PATH, '');
	const { branch, summary } = await git.commit(
		msg,
		undefined,
		commitOptions,
		async (data) => {
			if (data) {
				print(data);
				await unlink(TEMP_FILE_PATH);
				process.exit(1);
			}
		}
	);
	await unlink(TEMP_FILE_PATH);
	is.committing = false;
	printCommitSummary(branch, msg, summary);
	process.exit(0);
};

const onTerminate = async () => {
	if (is.committing) {
		await unlink('.git/index.lock');
		await unlink(TEMP_FILE_PATH);
		print('temporary files have been deleted');
	}
	process.exit(0);
};

process.on('SIGINT', onTerminate);

if (/\[(skip ci|ci skip)]/i.test(msg)) process.exit(0);

const git = simpleGit({ baseDir: process.cwd() });

const pkg = JSON.parse(await readFile('package.json', 'utf8'));

if (JSON.parse(await git.show(['main:package.json'])).version !== pkg.version)
	await commit();

const updateVersion = (version) => {
	pkg.version = version;
	return writeFile(
		'package.json',
		JSON.stringify(pkg, null, '\t'),
		'utf8'
	).then(() => {
		print('Version’s been updated to ' + pkg.version);
	});
};

const versionArr = pkg.version.split('.');
const v = {
	patch: `${versionArr[0]}.${versionArr[1]}.${+versionArr[2] + 1}`,
	minor: `${versionArr[0]}.${+versionArr[1] + 1}.0`,
	major: `${+versionArr[0] + 1}.0.0`,
};

const options = [
	['patch release', () => updateVersion(v.patch), v.patch],
	['minor release', () => updateVersion(v.minor), v.minor],
	['major release', () => updateVersion(v.major), v.major],
	[
		'skip ci',
		() => {
			msg = '[skip ci] ' + msg;
			print('Added [skip ci] to the commit message');
		},
	],
	[
		'cancel',
		() => {
			print('Canceled');
			process.exit(0);
		},
	],
];

const optionNames = options.map((item) => item[0]);
const optionNumber = optionNames.length;

let currOptionIndex = 0;
const moveMenuCursor = (step) => {
	process.stdout.clearLine();
	process.stdout.write('  ' + optionNames[currOptionIndex]);
	currOptionIndex += step;
	if (currOptionIndex === -1) {
		currOptionIndex = optionNumber - 1;
		step += optionNumber;
	} else if (currOptionIndex === optionNumber) {
		currOptionIndex = 0;
		step += -optionNumber;
	}
	process.stdout.moveCursor(0, step);
	const option = options[currOptionIndex];
	process.stdout.cursorTo(0);
	process.stdout.write(
		`> \x1b[34;4m${
			option[0] + (option[2] ? '\x1b[0;90m ' + option[2] : '')
		}\x1b[0m`
	);
	process.stdout.cursorTo(0);
};

print(
	'Version hasn’t been changed and there is no [skip ci] in commit message, what to do?\n' +
		optionNames.join('\n  ')
);
process.stdout.moveCursor(0, -optionNumber);
moveMenuCursor(0);

process.stdin.setRawMode(true);
process.stdin.resume();
process.stdin.setEncoding('utf8');
process.stdin.on('data', async (key) => {
	if (key === '\u0003') {
		if (is.inputProcessing)
			process.stdout.moveCursor(0, optionNumber - currOptionIndex);
		print('Terminated by Ctrl+C');
		await onTerminate();
		process.exit(0);
	}
	if (!is.inputProcessing) return;
	if (key === '\u000D' || key === ' ') {
		is.inputProcessing = false;
		process.stdout.moveCursor(0, optionNumber - currOptionIndex);
		await options[currOptionIndex][1]();
		await commit();
	}
	if (/^\u001B\u005B/.test(key)) {
		const char = key[2];
		if (char === '\u0041' || char === '\u0044') {
			moveMenuCursor(-1);
		} else if (char === '\u0042' || char === '\u0043') {
			moveMenuCursor(1);
		}
	}
});