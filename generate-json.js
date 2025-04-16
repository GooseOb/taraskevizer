import {
	alphabets,
	gobj,
	wordlist,
	softeners,
	noSoften,
} from './dist/dict/index.js';
import { resolve } from 'node:path';
import { writeFile, mkdir } from 'node:fs/promises';

const extractDict = (dict) =>
	dict.value.map(({ 0: pattern, 1: result }) => [pattern.source, result]);

const outDir = resolve('json');

const myAlphabets = {};

for (const key in alphabets) {
	const abc = alphabets[key];
	const myAbc = {
		lower: extractDict(abc.lower),
	};
	if (abc.upper) myAbc.upper = extractDict(abc.upper);
	myAlphabets[key] = myAbc;
}

await mkdir(outDir).catch(() => {});

await Promise.all(
	[
		['alphabets', myAlphabets],
		['g', gobj],
		['wordlist', extractDict(wordlist)],
		['softeners', extractDict(softeners)],
		['noSoften', extractDict(noSoften)],
	].map(({ 0: name, 1: obj }) =>
		writeFile(resolve(outDir, `${name}.json`), JSON.stringify(obj))
	)
);
