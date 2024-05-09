import { readFile } from 'fs/promises';
import { benchmark } from './lib';
import { Taraskevizer } from '../src';

const text = await readFile('test/large-text.txt', 'utf8').catch((e) => {
	console.error(e);
	process.exit(1);
});

const taraskevizer = new Taraskevizer();

benchmark('Taraskevization', () => {
	taraskevizer.convert(text);
});
