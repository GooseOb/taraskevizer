#!/usr/bin/env node
import {
	dicts,
	REPLACE_J,
	VARIATION,
	TaraskConfig,
	tarask,
	pipelines,
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

type Mode = keyof typeof pipelines;
let mode: Mode = 'plainText';

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
			general.abc = dicts.alphabets.latin;
		},
	],
	[
		['--latin-ji', '-lj'],
		() => {
			general.abc = dicts.alphabets.latinJi;
		},
	],
	[
		['--arabic', '-a'],
		() => {
			general.abc = dicts.alphabets.arabic;
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
			mode = 'abcOnly';
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

if (process.stdout.write(tarask(text, pipelines[mode], cfg) + '\n')) {
	process.exit(0);
} else {
	process.stdout.once('drain', () => {
		process.exit(0);
	});
}
