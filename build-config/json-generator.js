import path from 'path';
import { writeFile, mkdir } from 'fs/promises';
const outputPath = path.resolve('json');

const regexToStr = (dict) => {
	for (const item of dict) item[0] = item[0].source;
	return dict;
};

export default () =>
	mkdir(outputPath)
		.catch(() => null)
		.then(() => import('../src/dict/index.js'))
		.then(
			({
				wordlist,
				softers,
				latinLetters,
				latinLettersUpperCase,
				gobj,
				greekLetters,
				greekLettersUpperCase,
				thWords,
			}) =>
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
