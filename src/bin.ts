#!/usr/bin/env node
import { ALPHABET, REPLACE_J, Taraskevizer, VARIATION } from './index.js';
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

const general: Partial<TaraskOptions> = {};
const nonHtml: Partial<NonHtmlOptions> = {
	variations: 2,
	ansiColors: true,
};
const html: Partial<HtmlOptions> = { g: true };

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
			general.abc = ALPHABET.LATIN;
		},
	],
	[
		['--arabic', '-a'],
		() => {
			general.abc = ALPHABET.ARABIC;
		},
	],
	[
		['--jrandom', '-jr'],
		() => {
			general.j = REPLACE_J.RANDOM;
		},
	],
	[
		['--jalways', '-ja'],
		() => {
			general.j = REPLACE_J.ALWAYS;
		},
	],
	[
		['--no-escape-caps', '-nec'],
		() => {
			general.doEscapeCapitalized = false;
		},
	],
	[
		['--h', '-h'],
		() => {
			nonHtml.h = true;
			html.g = false;
		},
	],
	[
		['--no-variations', '-nv'],
		() => {
			nonHtml.variations = VARIATION.NO;
		},
	],
	[
		['--first-variation-only', '-fvo'],
		() => {
			nonHtml.variations = VARIATION.FIRST;
		},
	],
	[
		['--no-color', '-nc'],
		() => {
			nonHtml.ansiColors = false;
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

let text = '';
if (process.argv.length) {
	text = process.argv.join(' ');
} else {
	if (process.stdin.isTTY) {
		text = await readline
			.createInterface({
				input: process.stdin,
				output: process.stdout,
			})
			.question(prefix + 'Enter the text:\n');
	} else {
		for await (const chunk of process.stdin) text += chunk;
	}
}

const taraskevizer = new Taraskevizer({ general, html, nonHtml });

process.stdout.write(
	(isHtml ? taraskevizer.convertToHtml(text) : taraskevizer.convert(text)) +
		'\n'
);
process.exit(0);
