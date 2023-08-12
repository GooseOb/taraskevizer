import { benchmark, print, test } from './lib.js';
import { readFile } from 'node:fs/promises';
import { tarask, taraskSync } from '../dist/index.js';

const doBenchmarks = process.argv.includes('--benchmark');

print('test', 'start', '35');

test('Taraskevization', taraskSync, [
	['жыццясцвярджальны план', 'жыцьцясьцьвярджальны плян'],
	['варэнне', 'варэньне'],
]);

test('Sync and async functions return the same result', taraskSync, [
	['жыццясцвярджальны план', await tarask('жыццясцвярджальны план')],
]);

test('HtmlOptions', ([text, html]) => taraskSync(text, { html }), [
	[
		['жыццясцвярджальны план', {}],
		'жыц<tarF>ьцясьць</tarF>вярджальны пл<tarF>я</tarF>н',
	],
	[['газета', { g: false }], '<tarF><tarH>г</tarH></tarF>аз<tarF>э</tarF>та'],
	[['газета', { g: true }], '<tarF><tarH>ґ</tarH></tarF>аз<tarF>э</tarF>та'],
]);

print('test', 'all tests passed successfully', '35');

if (doBenchmarks) {
	const text = await readFile('test/large-text.txt', 'utf8');
	benchmark('Taraskevization', () => {
		taraskSync(text);
	});
}
