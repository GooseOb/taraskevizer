#!/usr/bin/env node
import {
	dicts,
	TaraskConfig,
	tarask,
	pipelines,
	htmlConfigOptions,
	wrappers,
} from '.';
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

let cfg = {
	g: true,
	variations: 'all',
	wrapperDict: wrappers.ansiColor,
} as TaraskConfig;

let mode: keyof typeof pipelines = 'tar';

const toHashTable = (
	dict: readonly (readonly [readonly string[], () => void])[]
) => {
	const result: Record<string, () => void> = {};
	for (const [options, callback] of dict)
		for (const option of options) result[option] = callback;
	return result;
};

let isHtml = false;

const optionDict = toHashTable([
	[['--help', '-h'], () => {}],
	[
		['--latin', '-l'],
		() => {
			cfg.abc = dicts.alphabets.latin;
		},
	],
	[
		['--latin-ji', '-lj'],
		() => {
			cfg.abc = dicts.alphabets.latinJi;
		},
	],
	[
		['--arabic', '-a'],
		() => {
			cfg.abc = dicts.alphabets.arabic;
		},
	],
	[
		['--jrandom', '-jr'],
		() => {
			cfg.j = 'random';
		},
	],
	[
		['--jalways', '-ja'],
		() => {
			cfg.j = 'always';
		},
	],
	[
		['--no-escape-caps', '-nec'],
		() => {
			cfg.doEscapeCapitalized = false;
		},
	],
	[
		['--h'],
		() => {
			cfg.g = false;
		},
	],
	[
		['--no-variations', '-nv'],
		() => {
			cfg.variations = 'no';
		},
	],
	[
		['--first-variation', '-fv'],
		() => {
			cfg.variations = 'first';
		},
	],
	[
		['--no-color', '-nc'],
		() => {
			cfg.wrapperDict = null;
		},
	],
	[
		['--html', '-html'],
		() => {
			isHtml = true;
		},
	],
	[
		['--alphabet-only', '-abc'],
		() => {
			mode = 'abc';
		},
	],
	[
		['--phonetic', '-ph'],
		() => {
			mode = 'phonetic';
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
	const chunks: Uint8Array[] = [];
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

cfg = new TaraskConfig(
	isHtml
		? {
				...htmlConfigOptions,
				...cfg,
				wrapperDict: htmlConfigOptions.wrapperDict,
			}
		: cfg
);

if (process.stdout.write(tarask(text, pipelines[mode], cfg) + '\n')) {
	process.exit(0);
} else {
	process.stdout.once('drain', () => {
		process.exit(0);
	});
}
