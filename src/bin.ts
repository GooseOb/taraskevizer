#!/usr/bin/env node
import {
	dicts,
	REPLACE_J,
	VARIATION,
	TaraskConfig,
	tarask,
	pipelines,
} from './index.js';
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

const cfg = new TaraskConfig({
	general: {},
	html: { g: true },
	nonHtml: { variations: VARIATION.ALL, ansiColors: true },
});

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
			cfg.general.abc = dicts.alphabets.latin;
		},
	],
	[
		['--latin-ji', '-lj'],
		() => {
			cfg.general.abc = dicts.alphabets.latinJi;
		},
	],
	[
		['--arabic', '-a'],
		() => {
			cfg.general.abc = dicts.alphabets.arabic;
		},
	],
	[
		['--jrandom', '-jr'],
		() => {
			cfg.general.j = REPLACE_J.RANDOM;
		},
	],
	[
		['--jalways', '-ja'],
		() => {
			cfg.general.j = REPLACE_J.ALWAYS;
		},
	],
	[
		['--no-escape-caps', '-nec'],
		() => {
			cfg.general.doEscapeCapitalized = false;
		},
	],
	[
		['--h'],
		() => {
			cfg.nonHtml.h = true;
			cfg.html.g = false;
		},
	],
	[
		['--no-variations', '-nv'],
		() => {
			cfg.nonHtml.variations = VARIATION.NO;
		},
	],
	[
		['--first-variation-only', '-fvo'],
		() => {
			cfg.nonHtml.variations = VARIATION.FIRST;
		},
	],
	[
		['--no-color', '-nc'],
		() => {
			cfg.nonHtml.ansiColors = false;
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

process.argv.reverse();
while ((currOption = process.argv.pop())) {
	if (currOption in optionDict) {
		optionDict[currOption]();
	} else {
		process.argv.push(currOption);
		break;
	}
}

let text = '';
if (process.argv.length) {
	text = process.argv.reverse().join(' ');
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

if (process.stdout.write(tarask(text, pipelines[mode], cfg) + '\n')) {
	process.exit(0);
} else {
	process.stdout.once('drain', () => {
		process.exit(0);
	});
}
