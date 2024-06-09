import { readFile } from 'fs/promises';
import { benchmark } from './lib';
import { tarask, pipelines } from '../src';

const text = await readFile('test/large-text.txt', 'utf8').catch((e) => {
	console.error(e);
	process.exit(1);
});

benchmark('Taraskevization', () => {
	tarask(text, pipelines.plainText);
});
