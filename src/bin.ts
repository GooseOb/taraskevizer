#!/usr/bin/env node
import { dicts, TaraskConfig, pipelines, htmlConfigOptions, wrappers } from '.';
declare const __CLI_HELP__: string;
declare const __CLI_PREFIX__: string;
declare const __VERSION__: string;

const printWithPrefix = (msg: string) => {
	process.stdout.write(__CLI_PREFIX__ + ' ' + msg + '\n');
};

process.argv.splice(0, 2);

const firstArg = process.argv[0];

if (firstArg) {
	if (firstArg === '-v' || firstArg === '--version') {
		printWithPrefix(__VERSION__);
		process.exit(0);
	}

	if (firstArg === '-h' || firstArg === '--help') {
		printWithPrefix(__CLI_HELP__);
		process.exit(0);
	}
}

let cfg: Partial<TaraskConfig> = {
	g: true,
	variations: 'all',
	wrappers: wrappers.ansiColor,
};

let mode: keyof typeof pipelines = 'tarask';

const toHashTable = (
	dict: readonly (readonly [readonly string[], () => void])[]
) => {
	const result: Record<string, () => void> = {};
	for (const { 0: options, 1: callback } of dict)
		for (const option of options) result[option] = callback;
	return result;
};

let isHtml = false;

const optionDict = toHashTable([
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
			cfg.wrappers = null;
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
			mode = 'alphabetic';
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
				wrappers: htmlConfigOptions.wrappers,
			}
		: cfg
);

if (process.stdout.write(pipelines[mode](text, cfg) + '\n')) {
	process.exit(0);
} else {
	process.stdout.once('drain', () => {
		process.exit(0);
	});
}
