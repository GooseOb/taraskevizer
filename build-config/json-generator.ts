import * as path from 'path';
import { writeFile, mkdir } from 'node:fs/promises';
import { dicts } from '../src';
const outputPath = path.resolve('json');

const dictToSerializeble = (dict: readonly (readonly [RegExp, string])[]) =>
	dict.map(([pattern, result]) => [pattern.source, result]);

export default () =>
	mkdir(outputPath)
		.catch((err) => {
			if (err.code !== 'EEXIST') throw err;
		})
		.then(() => {
			const alphabetsSerializeble: any = dicts.alphabets;
			for (const key in alphabetsSerializeble) {
				const item = alphabetsSerializeble[key];
				item.lower = dictToSerializeble(item.lower);
				if (item.upper) item.upper = dictToSerializeble(item.upper);
			}
			return Promise.all(
				[
					...(
						[
							'wordlist',
							'softeners',
							'noSoften',
						] satisfies (keyof typeof dicts)[]
					).map((name) => [
						name,
						JSON.stringify(dictToSerializeble(dicts[name])),
					]),
					['g', JSON.stringify(dicts.gobj)],
					['alphabets', JSON.stringify(dicts.alphabets)],
				].map(([fileName, str]) =>
					writeFile(path.resolve(outputPath, fileName + '.json'), str)
				)
			);
		});
