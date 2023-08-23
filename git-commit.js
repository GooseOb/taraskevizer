import { readFile, writeFile } from 'node:fs/promises';
import simpleGit from 'simple-git';

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

if (/\[(skip ci|ci skip)]/i.test(msg)) process.exit(0);

const git = simpleGit({ baseDir: process.cwd() });

const pkg = JSON.parse(await readFile('package.json', 'utf8'));

if (JSON.parse(await git.show(['main:package.json'])).version !== pkg.version)
	process.exit(0);

const updateVersion = (version) => {
	pkg.version = version;
	return writeFile(
		'package.json',
		JSON.stringify(pkg, null, '\t'),
		'utf8'
	).then(() => {
		printUnderMenu('Version’s been updated to ' + pkg.version);
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
			printUnderMenu('Added [skip ci] to the commit message');
		},
	],
	[
		'cancel',
		() => {
			printUnderMenu('Canceled');
			exit();
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
		process.stdout.moveCursor(0, optionNumber);
	} else if (currOptionIndex === optionNumber) {
		currOptionIndex = 0;
		process.stdout.moveCursor(0, -optionNumber);
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
const printUnderMenu = (...msgs) => {
	process.stdout.moveCursor(0, optionNumber - currOptionIndex);
	print(...msgs);
	process.stdout.moveCursor(0, -currOptionIndex + 2);
};
const exit = () => {
	process.stdout.moveCursor(0, optionNumber - currOptionIndex);
	process.exit(0);
};

const commit = async () => {
	process.stdout.moveCursor(0, optionNumber - currOptionIndex);
	const { branch, summary } = await git.commit(
		msg,
		undefined,
		commitOptions,
		(data) => {
			if (data) {
				print(data);
				process.exit(1);
			}
		}
	);
	print(
		`${branch}: ${msg}\nchanged files: \x1b[33m${summary.changes}\x1b[0m insertions: \x1b[32m${summary.insertions}\x1b[0m deletions: \x1b[31m${summary.deletions}\x1b[0m`
	);
	process.exit(0);
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
process.stdin.on('data', (key) => {
	if (key === '\u0003') {
		printUnderMenu('Terminated by Ctrl+C');
		exit();
	}
	if (key === '\u000D' || key === ' ') {
		const result = options[currOptionIndex][1]();
		if (typeof result === 'object' && result.then) result.then(commit);
		else commit();
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
