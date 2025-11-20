import { readFile } from 'node:fs/promises';
import { pipelines } from '../src';
import { benchmark } from './lib';

const text = await readFile('test/texts/large.txt', 'utf8').catch((e) => {
	console.error(e);
	process.exit(1);
});

benchmark('Taraskevization', () => {
	pipelines.tarask(text);
});
