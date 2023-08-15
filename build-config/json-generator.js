import path from 'path';
import { exists as _exists } from 'fs';
import { promisify } from 'util';
import { writeFile, mkdir } from 'fs/promises';
const exists = promisify(_exists);
const outputPath = path.resolve('json');
import {
	wordlist,
	softers,
	latinLetters,
	latinLettersUpperCase,
	gobj,
	greekLetters,
	greekLettersUpperCase,
	thWords,
} from '../src/dict/index.js';

const regexToStr = (dict) => {
	for (const item of dict) item[1] = item[1].source;
	return dict;
};

let beenExecuted = false;
export default (execOnlyOnce) => {
	if (execOnlyOnce) {
		if (beenExecuted) return;
		beenExecuted = true;
	}
	exists(outputPath)
		.then((exists) => exists || mkdir(outputPath))
		.then(() =>
			Promise.all(
				[
					['wordlist', regexToStr(wordlist)],
					['softers', regexToStr(softers)],
					['latinLetters', regexToStr(latinLetters)],
					['latinLettersUpperCase', regexToStr(latinLettersUpperCase)],
					['greekLetters', regexToStr(greekLetters)],
					['greekLettersUpperCase', regexToStr(greekLettersUpperCase)],
					['thWords', regexToStr(thWords)],
					['g', gobj],
				].map(([fileName, obj]) =>
					writeFile(
						path.resolve(outputPath, fileName + '.json'),
						JSON.stringify(obj)
					)
				)
			)
		);
};
