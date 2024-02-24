import * as path from 'path';
import { writeFile, mkdir } from 'node:fs/promises';
import {
	wordlist,
	softeners,
	latinLetters,
	latinLettersUpperCase,
	gobj,
	greekLetters,
	greekLettersUpperCase,
	thWords,
} from '../src/dict';
import type { Dict } from '../src/types';
const outputPath = path.resolve('json');

const dictToStr = (dict: Dict) =>
	JSON.stringify(dict.map(([pattern, result]) => [pattern.source, result]));

export default () =>
	mkdir(outputPath)
		.catch((err) => {
			if (err.code !== 'EEXIST') throw err;
		})
		.then(() =>
			Promise.all(
				[
					['wordlist', dictToStr(wordlist)],
					['softeners', dictToStr(softeners)],
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
