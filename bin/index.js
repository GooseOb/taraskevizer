#!/usr/bin/env node
import { tarask, taraskToHtml } from '../dist/index.js';
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

/** @type {Partial<import('../dist/types.js').TaraskOptions>} */
const taraskOptions = {};
/** @type {Partial<import('../dist/types.js').NonHtmlOptions>} */
const nonHtmlOptions = { variations: 2, nodeColors: true };
/** @type {Partial<import('../dist/types.js').HtmlOptions>} */
const htmlOptions = { g: true };

let isHtml = false;

const optionDict = [
	[
		['--latin', '-l'],
		() => {
			taraskOptions.abc = 1;
		},
	],
	[
		['--arabic', '-a'],
		() => {
			taraskOptions.abc = 2;
		},
	],
	[
		['--greek', '-gr'],
		() => {
			taraskOptions.abc = 3;
		},
	],
	[
		['--jrandom', '-jr'],
		() => {
			taraskOptions.j = 1;
		},
	],
	[
		['--jalways', '-ja'],
		() => {
			taraskOptions.j = 2;
		},
	],
	[
		['--h', '-h'],
		() => {
			nonHtmlOptions.h = true;
			htmlOptions.g = false;
		},
	],
	[
		['--no-variations', '-nv'],
		() => {
			nonHtmlOptions.variations = 0;
		},
	],
	[
		['--first-variation-only', '-fvo'],
		() => {
			nonHtmlOptions.variations = 1;
		},
	],
	[
		['--no-color', '-nc'],
		() => {
			nonHtmlOptions.nodeColors = false;
		},
	],
	[
		['--html', '-html'],
		() => {
			isHtml = true;
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

console.log(
	isHtml
		? taraskToHtml(text, taraskOptions, htmlOptions)
		: tarask(text, taraskOptions, nonHtmlOptions)
);
