#!/usr/bin/env node
import { taraskSync } from '../dist/index.js';
import { readFile } from 'fs/promises';

const print = (...msgs) => {
	console.log('\x1b[34m[taraskevizer]\x1b[0m', ...msgs);
};

const getPkg = () =>
	readFile(new URL('../package.json', import.meta.url)).then(JSON.parse);

process.argv.splice(0, 2);

if (['-v', '--version'].includes(process.argv[0])) {
	const { version } = await getPkg();
	print(version);
	process.exit(0);
}

const stgs = {
	html: {},
};
let noVariations = false;
let firstVariationOnly = false;
let noColors = false;

const optionDict = [
	[
		['--latin', '-l'],
		() => {
			stgs.abc = 1;
		},
	],
	[
		['--arabic', '-a'],
		() => {
			stgs.abc = 2;
		},
	],
	[
		['--jrandom', '-jr'],
		() => {
			stgs.j = 1;
		},
	],
	[
		['--jalways', '-ja'],
		() => {
			stgs.j = 2;
		},
	],
	[
		['--g', '-g'],
		() => {
			stgs.html.g = true;
		},
	],
	[
		['--no-variations', '-nv'],
		() => {
			noVariations = true;
		},
	],
	[
		['--first-variation-only', '-fvo'],
		() => {
			firstVariationOnly = true;
		},
	],
	[
		['--no-colors', '-nc'],
		() => {
			noColors = true;
		},
	],
];

optionEater: while (true) {
	for (const [options, callback] of optionDict)
		if (options.includes(process.argv[0])) {
			process.argv.shift();
			callback();
			continue optionEater;
		}
	break;
}

const text = process.argv.join(' ');
if (!text) process.exit(0);

const htmlTagsToNodeColors = (text) => {
	text = text.replace(
		/<tarL data-l='(.*?)'>(.*?)<\/tarL>/g,
		firstVariationOnly
			? ($0, $1) => `\x1b[35m${$1.split(',')[0]}\x1b[0m`
			: noVariations
			? '\x1b[35m$2\x1b[0m'
			: ($0, $1, $2) => `\x1b[35m(${$2}|${$1.replace(/,/, '|')})\x1b[0m`
	);
	return noColors
		? text.replace(/<\/?tar[FH]>|\x1b\[\d+?m/g, '')
		: text
				.replace(/<tarF>/g, '\x1b[32m')
				.replace(/<tarH>/g, '\x1b[35m')
				.replace(/<\/tar[FH]>/g, '\x1b[0m');
};

console.log(htmlTagsToNodeColors(taraskSync(text, stgs)));
