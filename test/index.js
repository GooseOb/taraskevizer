import { readFile } from 'node:fs/promises';
import { benchmark, print, test } from './lib.js';
import { tarask, taraskSync, ALPHABET, J } from '../dist/index.js';

const doBenchmarks = process.argv.includes('--benchmark');

print('test', 'start', '35');

test('Taraskevization', taraskSync, [
	['жыццясцвярджальны план', 'жыцьцясьцьвярджальны плян'],
	['варэнне', 'варэньне'],
	['секцыя селекцыйных селектараў', 'сэкцыя сэлекцыйных сэлектараў'],
]);

test('Sync and async functions return the same result', taraskSync, [
	['жыццясцвярджальны план', await tarask('жыццясцвярджальны план')],
]);

test('HtmlOptions', ([text, html]) => taraskSync(text, { html }), [
	[
		['жыццясцвярджальны план', {}],
		'жыц<tarF>ьцясьць</tarF>вярджальны пл<tarF>я</tarF>н',
	],
	[['газета', { g: false }], '<tarH>г</tarH>аз<tarF>э</tarF>та'],
	[['газета', { g: true }], '<tarH>ґ</tarH>аз<tarF>э</tarF>та'],
]);

test('i -> j', ([text, j, abc]) => taraskSync(text, { j, abc }), [
	[['Яна і ён'], 'Яна і ён'],
	[['Яна і ён', J.NEVER], 'Яна і ён'],
	[['Яна і ён', J.ALWAYS], 'Яна й ён'],
	[['Яна і ён', undefined, ALPHABET.LATIN], 'Jana i jon'],
	[['Яна і ён', J.NEVER, ALPHABET.LATIN], 'Jana i jon'],
	[['Яна і ён', J.ALWAYS, ALPHABET.LATIN], 'Jana j jon'],
]);

test('greek th', (text) => taraskSync(text, { abc: ALPHABET.GREEK }), [
	['матэматычная кафедра', 'μαθεματίτσ̌ναυα καθεδρα'],
	['афінская арыфметыка', 'αθενσκαυα αρίθμετίκα'],
	['фесалонікі, салонікі', '(θεσ|σ)αλυονικι, (σ|θεσ)αλυονικι'],
	['Тэарэма Піфагора', 'Θεαρεμα Πιθαγορα'],
	['Тэатр арыфметычнага тэрматэізму', 'Θεαταρ αρίθμετίτσ̌ναγα Θερμαθειζμϋ'],
]);

print('test', 'all tests passed successfully', '35');

if (doBenchmarks) {
	const text = await readFile('test/large-text.txt', 'utf8');
	benchmark('Taraskevization', () => {
		taraskSync(text);
	});
}
