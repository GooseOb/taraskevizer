#!/usr/bin/env node
import { tarask, taraskToHtml } from './index.js';
import { readFile } from 'fs/promises';
const printWithPrefix = (msg) => {
	process.stdout.write(
		'\x1b[34m[taraskevizer]\x1b[0m ' + msg.toString() + '\n'
	);
};
process.argv.splice(0, 2);
const checkForOptions = (options) =>
	options.includes(process.argv[0].toLowerCase());
if (checkForOptions(['-v', '--version'])) {
	const { version } = JSON.parse(
		await readFile(new URL('../package.json', import.meta.url), 'utf8')
	);
	printWithPrefix(version);
	process.exit(0);
}
const taraskOptions = {};
const nonHtmlOptions = {
	variations: 2,
	ansiColors: true,
};
const htmlOptions = { g: true };
let isHtml = false;
const toHashTable = (dict) => {
	const result = {};
	for (const [options, callback] of dict)
		for (const option of options) result[option] = callback;
	return result;
};
const optionDict = toHashTable([
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
			nonHtmlOptions.ansiColors = false;
		},
	],
	[
		['--html', '-html'],
		() => {
			isHtml = true;
		},
	],
]);
let currOption;
while ((currOption = process.argv.shift())) {
	if (currOption in optionDict) {
		optionDict[currOption]();
	} else {
		process.argv.unshift(currOption);
		break;
	}
}
const text = process.argv.join(' ');
process.stdout.write(
	isHtml
		? taraskToHtml(text, taraskOptions, htmlOptions)
		: tarask(text, taraskOptions, nonHtmlOptions)
);
