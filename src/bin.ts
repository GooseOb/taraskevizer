#!/usr/bin/env node
import { tarask, taraskToHtml } from './index.js';
import readline from 'readline/promises';
import { readFile } from 'fs/promises';
import type { NonHtmlOptions, TaraskOptions, HtmlOptions } from './types';

const prefix = '\x1b[34m[taraskevizer]\x1b[0m ';
const printWithPrefix = (msg: string) => {
	process.stdout.write(prefix + msg.toString() + '\n');
};

process.argv.splice(0, 2);

const checkForOptions = (options: string[]) =>
	process.argv[0] && options.includes(process.argv[0].toLowerCase());

if (checkForOptions(['-v', '--version'])) {
	const { version } = JSON.parse(
		await readFile(new URL('../package.json', import.meta.url), 'utf8')
	);
	printWithPrefix(version);
	process.exit(0);
}

const taraskOptions: Partial<TaraskOptions> = {};
const nonHtmlOptions: Partial<NonHtmlOptions> = {
	variations: 2,
	ansiColors: true,
};
const htmlOptions: Partial<HtmlOptions> = { g: true };

let isHtml = false;

const toHashTable = (dict: [string[], () => void][]) => {
	const result: Record<string, () => void> = {};
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

let currOption: string | undefined;

while ((currOption = process.argv.shift())) {
	if (currOption in optionDict) {
		optionDict[currOption]();
	} else {
		process.argv.unshift(currOption);
		break;
	}
}

let text = process.argv.length
	? process.argv.join(' ')
	: await readline
			.createInterface({
				input: process.stdin,
				output: process.stdout,
			})
			.question(prefix + 'Enter the text:\n');

process.stdout.write(
	isHtml
		? taraskToHtml(text, taraskOptions, htmlOptions)
		: tarask(text, taraskOptions, nonHtmlOptions)
);
process.exit(0);
