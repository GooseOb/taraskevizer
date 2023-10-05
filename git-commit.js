import { readFile, writeFile, unlink } from 'node:fs/promises';
import path from 'node:path';
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
const removeFromCommitOptions = (from, to) => {
	commitOptions = commitOptions.slice(0, from).concat(commitOptions.slice(to));
};

const indexOfGpgTty = commitOptions.indexOf('--gpgtty');
const gpgTty = indexOfGpgTty !== -1;
if (gpgTty) {
	const { execSync } = await import('node:child_process');
	removeFromCommitOptions(indexOfGpgTty, indexOfGpgTty + 1);
	process.env.GPG_TTY = execSync('tty', {
		stdio: ['inherit', 'pipe', 'pipe'],
	})
		.toString()
		.trim();
}

for (let i = 0; i < commitOptions.length; i++) {
	const item = commitOptions[i];
	if (item === '-m' || item === '-am') {
		msg = commitOptions[i + 1];
		let from = i;
		if (item === '-am') {
			commitOptions[i] = '-a';
			from = i + 1;
		}
		removeFromCommitOptions(from, i + 2);
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
	print(`${branch}: ${msg}
changed files: \x1b[33m${changes}\x1b[0m insertions: \x1b[32m${insertions}\x1b[0m deletions: \x1b[31m${deletions}\x1b[0m
package version: ${pkg.version}`);

const TEMP_FILE_PATH = path.resolve('.git', 'commit-safe');
const onCommitError = (data) => {
	print(data);
	printOptions('An error occurred, what to do?', [
		['try again', commit],
		['exit', () => unlink(TEMP_FILE_PATH).then(() => process.exit(1))],
	]);
};
const commit = async () => {
	is.committing = true;
	await writeFile(TEMP_FILE_PATH, '');
	let commitResult;
	try {
		commitResult = await git.commit(msg, undefined, commitOptions, (data) => {
			if (data) onCommitError(data);
		});
	} catch (e) {
		onCommitError(e);
		return;
	}
	await unlink(TEMP_FILE_PATH);
	is.committing = false;
	printCommitSummary(commitResult.branch, msg, commitResult.summary);
	openPushMenu();
};

const openPushMenu = () => {
	printOptions('Make a push?', [
		['no', () => process.exit(0)],
		[
			'yes',
			() =>
				git
					.push()
					.then(({ repo, update: { head, hash } }) => {
						print('Pushed to the repo ' + repo);
						print(`${hash.from}..${hash.to} ${head.local} -> ${head.remote}`);
						process.exit(0);
					})
					.catch((err) => {
						console.log(err);
						process.exit(1);
					}),
		],
	]);
	is.inputProcessing = true;
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

const mainOptions = [
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

const moveMenuCursor = (step) => {
	process.stdout.clearLine();
	process.stdout.write('  ' + options.names[options.currIndex]);
	options.currIndex += step;
	if (options.currIndex === -1) {
		options.currIndex = options.length - 1;
		step += options.length;
	} else if (options.currIndex === options.length) {
		options.currIndex = 0;
		step += -options.length;
	}
	process.stdout.moveCursor(0, step);
	const option = options[options.currIndex];
	process.stdout.cursorTo(0);
	process.stdout.write(
		`> \x1b[34;4m${
			option[0] + (option[2] ? '\x1b[0;90m ' + option[2] : '')
		}\x1b[0m`
	);
	process.stdout.cursorTo(0);
};
let options;
const printOptions = (msg, arr) => {
	options = arr;
	options.names = arr.map((item) => item[0]);
	options.currIndex = 0;
	print(msg + '\n' + options.names.join('\n  '));
	process.stdout.moveCursor(0, -arr.length);
	moveMenuCursor(0);
	is.inputProcessing = true;
};

printOptions(
	'Version hasn’t been changed and there is no [skip ci] in commit message, what to do?',
	mainOptions
);

process.stdin.setRawMode(true);
process.stdin.resume();
process.stdin.setEncoding('utf8');
process.stdin.on(
	'data',
	/** @param key {string} */
	async (key) => {
		if (key === '\u0003') {
			if (is.inputProcessing)
				process.stdout.moveCursor(0, options.length - options.currIndex);
			print('Terminated by Ctrl+C');
			await onTerminate();
			process.exit(0);
		}
		if (!is.inputProcessing) return;
		if (key === '\u000D' || key === ' ') {
			is.inputProcessing = false;
			process.stdout.moveCursor(0, options.length - options.currIndex);
			await options[options.currIndex][1]();
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
	}
);
