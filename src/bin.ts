#!/usr/bin/env node
import readline from 'readline/promises';
import { readFileSync } from 'fs';
import { ALPHABET, REPLACE_J, Taraskevizer, VARIATION } from './index.js';
import type { NonHtmlOptions, TaraskOptions, HtmlOptions } from './types';
declare const __CLI_HELP__: string;
declare const __VERSION__: string;

const prefix = '\x1b[34m[taraskevizer]\x1b[0m ';
const printWithPrefix = (msg: string) => {
	process.stdout.write(prefix + msg + '\n');
};

process.argv.splice(0, 2);

const checkForOptions = (options: string[]) =>
	process.argv[0] && options.includes(process.argv[0].toLowerCase());

if (checkForOptions(['-v', '--version'])) {
	printWithPrefix(__VERSION__);
	process.exit(0);
}

if (checkForOptions(['-h', '--help'])) {
	printWithPrefix(__CLI_HELP__);
	process.exit(0);
}

const general: Partial<TaraskOptions> = {};
const nonHtml: Partial<NonHtmlOptions> = {
	variations: 2,
	ansiColors: true,
};
const html: Partial<HtmlOptions> = { g: true };

type Mode = 'nonHtml' | 'html' | 'alphabetOnly';
let mode: Mode = 'nonHtml';

const toHashTable = (dict: [string[], () => void][]) => {
	const result: Record<string, () => void> = {};
	for (const [options, callback] of dict)
		for (const option of options) result[option] = callback;
	return result;
};

const optionDict = toHashTable([
	[['--help', '-h'], () => {}],
	[
		['--latin', '-l'],
		() => {
			general.abc = ALPHABET.LATIN;
		},
	],
	[
		['--latin-ji', '-lj'],
		() => {
			general.abc = ALPHABET.LATIN_JI;
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
		['--h'],
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
			mode = 'html';
		},
	],
	[
		['--alphabet-only', '-abc'],
		() => {
			mode = 'alphabetOnly';
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
} else if (process.stdin.isTTY) {
	text = await readline
		.createInterface({
			input: process.stdin,
			output: process.stdout,
		})
		.question(prefix + 'Enter the text:\n');
} else {
	text = readFileSync(0, 'utf8');
}

const taraskevizer = new Taraskevizer({ general, html, nonHtml });

process.stdout.write(
	{
		nonHtml: taraskevizer.convert,
		html: taraskevizer.convertToHtml,
		alphabetOnly: taraskevizer.convertAlphabetOnly,
	}[mode].apply(taraskevizer, [text]) + '\n'
);
process.exit(0);
