import * as path from 'path';
import { writeFile, mkdir } from 'node:fs/promises';
import {
	wordlist,
	softers,
	latinLetters,
	latinLettersUpperCase,
	gobj,
	greekLetters,
	greekLettersUpperCase,
	thWords,
} from '../src/dict';
import { Dict } from '../src/types';
const outputPath = path.resolve('json');

const dictToStr = (dict: Dict) => {
	// @ts-ignore
	for (const item of dict) item[0] = item[0].source;
	return JSON.stringify(dict);
};

export default () =>
	mkdir(outputPath)
		.catch((err) => {
			if (err.code !== 'EEXIST') throw err;
		})
		.then(() =>
			Promise.all(
				[
					['wordlist', dictToStr(wordlist)],
					['softers', dictToStr(softers)],
					['latinLetters', dictToStr(latinLetters)],
					['latinLettersUpperCase', dictToStr(latinLettersUpperCase)],
					['greekLetters', dictToStr(greekLetters)],
					['greekLettersUpperCase', dictToStr(greekLettersUpperCase)],
					['thWords', dictToStr(thWords)],
					['g', JSON.stringify(gobj)],
				].map(([fileName, str]) =>
					writeFile(path.resolve(outputPath, fileName + '.json'), str)
				)
			)
		);
