#!/usr/bin/env node

const print = (...msgs) => {
	console.log('\x1b[34m[taraskevizer]\x1b[0m', ...msgs);
};

process.argv.shift();
process.argv.shift();

const checkForOptions = (options) => {
	for (const option of options)
		if (process.argv[0] === option) {
			process.argv.shift();
			return true;
		}
	return false;
};

if (checkForOptions(['-v', '--version'])) {
	print(require('../package.json').version);
	process.exit(0);
}

const stgs = {
	html: {},
};

const arr = [
	[
		['--latin', '-l'],
		() => {
			stgs.abc = 1;
		},
	],
	[
		['--arabic', '-a'],
		() => {
			stgs.abc = 2;
		},
	],
	[
		['--jrandom', '-jr'],
		() => {
			stgs.j = 1;
		},
	],
	[
		['--jalways', '-ja'],
		() => {
			stgs.j = 2;
		},
	],
	[
		['--g', '-g'],
		() => {
			stgs.j = 2;
		},
	],
];

opnionsEater: while (true) {
	for (const [options, callback] of arr)
		if (checkForOptions(options)) {
			callback();
			continue opnionsEater;
		}
	break;
}

const text = process.argv.join(' ');
if (!text) process.exit(0);

const { taraskSync } = require('../dist');

const htmlTagsToNodeColors = (text) =>
	text
		.replace(/<tarF>/g, '\x1b[32m')
		.replace(/<tar[LH].*?>/g, '\x1b[35m')
		.replace(/<\/\S+>/g, '\x1b[0m');

print(htmlTagsToNodeColors(taraskSync(text, stgs)));
