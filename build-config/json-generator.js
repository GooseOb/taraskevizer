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
} from '../src/dict.js';

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
