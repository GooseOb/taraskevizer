import * as path from 'path';
import { writeFile, mkdir } from 'node:fs/promises';
import * as dict from '../src/dict';
const outputPath = path.resolve('json');

export default () =>
	mkdir(outputPath)
		.catch((err) => {
			if (err.code !== 'EEXIST') throw err;
		})
		.then(() =>
			Promise.all(
				[
					...(
						[
							'wordlist',
							'softeners',
							'noSoften',
							'latinLettersLowerCase',
							'latinLettersLowerCaseJi',
							'latinLettersUpperCase',
							'latinLettersUpperCaseJi',
						] satisfies (keyof typeof dict)[]
					).map((name) => [
						name,
						JSON.stringify(
							dict[name].map(([pattern, result]) => [pattern.source, result])
						),
					]),
					['g', JSON.stringify(dict.gobj)],
				].map(([fileName, str]) =>
					writeFile(path.resolve(outputPath, fileName + '.json'), str)
				)
			)
		);
