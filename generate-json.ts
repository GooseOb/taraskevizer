import {
	alphabets,
	gobj,
	wordlist,
	softeners,
	noSoften,
	type CallableDict,
	type Dict,
} from './src/dict';
import { resolve } from 'path';
import { writeFile, mkdir } from 'fs/promises';

const extractDict = (dict: CallableDict): Dict<string> =>
	dict.value.map(({ 0: pattern, 1: result }) => [pattern.source, result]);

const postprocess = (code: string) =>
	code.replace(/\\\\u([A-Fa-f\d]{4})/g, (_$0, $1) =>
		String.fromCharCode(parseInt($1, 16))
	);

const outDir = resolve('json');

const myAlphabets = {};

for (const key in alphabets) {
	const abc = alphabets[key];
	const myAbc: { lower: Dict<string>; upper?: Dict<string> } = {
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
		writeFile(resolve(outDir, `${name}.json`), postprocess(JSON.stringify(obj)))
	)
);
