import { exec as execSync } from 'node:child_process';
import { promisify } from 'node:util';
import { readFile, writeFile } from 'node:fs/promises';
import readline from 'node:readline/promises';

const pathToMsg = process.argv[2];
const msg = await readFile(pathToMsg, 'utf8');

const isCISkipped = /\[(skip ci|ci skip)]/i.test(msg);
if (isCISkipped) process.exit(1);

const exec = promisify(execSync);

const { stdout } = await exec('git show main:package.json');
const { version } = JSON.parse(stdout);

const pkg = JSON.parse(await readFile('package.json', 'utf8'));

const prefix = '\x1b[35m[version-manager]\x1b[0m';
const print = (...msgs) => {
	console.log(prefix, ...msgs);
};

const updateVersion = (callback) => {
	const versionArr = pkg.version.split('.');
	callback(versionArr);
	pkg.version = versionArr.join('.');
	return writeFile(
		'package.json',
		JSON.stringify(pkg, null, '\t'),
		'utf8'
	).then(() => {
		print('Version’s been updated to ' + pkg.version);
	});
};

const options = {
	'+': {
		label: 'patch release',
		callback: () =>
			updateVersion((arr) => {
				++arr[2];
			}),
	},
	'++': {
		label: 'minor release',
		callback: () =>
			updateVersion((arr) => {
				++arr[1];
			}),
	},
	'+++': {
		label: 'major release',
		callback: () =>
			updateVersion((arr) => {
				++arr[0];
			}),
	},
	s: {
		label: 'skip ci',
		callback: () =>
			writeFile(pathToMsg, '[skip ci] ' + msg).then(() => {
				print('Added [skip ci] to the commit message');
			}),
	},
};

const optionsAsArray = Object.entries(options);

const maxOptionLength = optionsAsArray.reduce(
	(acc, [item]) => Math.max(item.length, acc),
	0
);

const optionsAsText =
	optionsAsArray.reduce(
		(acc, [key, { label }]) =>
			acc +
			key +
			') ' +
			' '.repeat(maxOptionLength - key.length) +
			label +
			'\n',
		`\n${prefix} Version hasn’t been changed and there is no [skip ci] in commit message, what to do?\n`
	) + 'To cancel commit, enter any other value\n\n> ';

if (version === pkg.version) {
	const rl = readline.createInterface({
		input: process.stdin,
		output: process.stdout,
	});
	const chosenOption = options[(await rl.question(optionsAsText)).trim()];
	rl.close();
	if (chosenOption) {
		await chosenOption.callback();
	} else {
		print('Commit’s been canceled');
		process.exit(1);
	}
}
