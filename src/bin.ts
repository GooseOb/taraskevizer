#!/usr/bin/env node
import {
	ALPHABET,
	REPLACE_J,
	VARIATION,
	TaraskConfig,
	tarask,
	abcOnlyPipeline,
	htmlPipeline,
	plainTextPipeline,
} from './index.js';
import type { NonHtmlOptions, TaraskOptions, HtmlOptions } from './types';
declare const __CLI_HELP__: string;
declare const __VERSION__: string;

const prefix = '\x1b[34m[taraskevizer]\x1b[0m ';
const printWithPrefix = (msg: string) => {
	process.stdout.write(prefix + msg + '\n');
};

process.argv.splice(0, 2);

const checkForOptions = (options: readonly string[]) =>
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

const toHashTable = (
	dict: readonly (readonly [readonly string[], () => void])[]
) => {
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
} else {
	const chunks = [];
	let length = 0;

	if (process.stdin.isTTY) {
		printWithPrefix('Enter the text');
		for await (const chunk of process.stdin) {
			chunks.push(chunk);
			length += chunk.length;
			if (chunk.includes('\n')) break;
		}
	} else {
		for await (const chunk of process.stdin) {
			chunks.push(chunk);
			length += chunk.length;
		}
	}

	text = Buffer.concat(chunks, length).toString();
}

const cfg = new TaraskConfig({ general, html, nonHtml });

const piplineByMode = {
	nonHtml: plainTextPipeline,
	html: htmlPipeline,
	alphabetOnly: abcOnlyPipeline,
};

if (process.stdout.write(tarask(text, piplineByMode[mode], cfg) + '\n')) {
	process.exit(0);
} else {
	process.stdout.once('drain', () => {
		process.exit(0);
	});
}
