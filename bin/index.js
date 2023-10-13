#!/usr/bin/env node
import { tarask } from '../dist/index.js';
import { readFile } from 'fs/promises';

const print = (...msgs) => {
	console.log('\x1b[34m[taraskevizer]\x1b[0m', ...msgs);
};

const getPkg = () =>
	readFile(new URL('../package.json', import.meta.url)).then(JSON.parse);

process.argv.splice(0, 2);

const checkForOptions = (options) =>
	options.includes(process.argv[0].toLowerCase());

if (checkForOptions(['-v', '--version'])) {
	const { version } = await getPkg();
	print(version);
	process.exit(0);
}

const stgs = {
	nonHtml: {
		variations: 2,
		nodeColors: true,
	},
};

const optionDict = [
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
		['--greek', '-gr'],
		() => {
			stgs.abc = 3;
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
		['--h', '-h'],
		() => {
			stgs.nonHtml.h = true;
		},
	],
	[
		['--no-variations', '-nv'],
		() => {
			stgs.nonHtml.variations = 0;
		},
	],
	[
		['--first-variation-only', '-fvo'],
		() => {
			stgs.nonHtml.variations = 1;
		},
	],
	[
		['--no-color', '-nc'],
		() => {
			stgs.nonHtml.nodeColors = false;
		},
	],
	[
		['--html', '-html'],
		() => {
			stgs.html = {};
		},
	],
];

optionEater: while (true) {
	for (const [options, callback] of optionDict)
		if (checkForOptions(options)) {
			process.argv.shift();
			callback();
			continue optionEater;
		}
	break;
}

const text = process.argv.join(' ');

console.log(tarask(text, stgs));
