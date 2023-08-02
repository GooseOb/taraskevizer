import path from 'path';
import { exists as _exists } from 'fs';
import { promisify } from 'util';
import { writeFile, mkdir } from 'fs/promises';
const exists = promisify(_exists);
const outputPath = path.resolve('json');

const regexToStr = (obj) => {
	for (const key in obj) obj[key] = obj[key].source;
	return obj;
};

let beenExecuted = false;
export default (source, execOnlyOnce) => {
	if (execOnlyOnce) {
		if (beenExecuted) return;
		beenExecuted = true;
	}
	let wordlist, softers, gobj, latinLetters, latinLettersUpperCase;
	eval(source.replace(/const|let|var|exports\.|"use strict";/g, ''));
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
