import { benchmark, print, test } from './lib.js';
import { readFile } from 'node:fs/promises';
import { tarask, taraskSync } from '../dist/index.js';

const doBenchmarks = process.argv.includes('--benchmark');

print('test', 'start', '35');

test('Taraskevization', taraskSync, [
	['жыццясцвярджальны план', 'жыцьцясьцьвярджальны плян'],
	['варэнне', 'варэньне'],
	['планета', 'плянэта'],
]);

test('Sync and async functions return the same result', taraskSync, [
	['жыццясцвярджальны план', await tarask('жыццясцвярджальны план')],
]);

print('test', 'all tests passed successfully', '35');

if (doBenchmarks) {
	const text = await readFile('test/large-text.txt', 'utf8');
	await benchmark('Taraskevization', () => {
		taraskSync(text);
	});
}
