import { benchmark, print, getTestProcess } from './lib';
import { taraskSync, ALPHABET } from '../src';
import * as cases from './cases';

print('test', 'start', '35');

const { summary, test } = getTestProcess();

test('Taraskevization', taraskSync, cases.taraskevization);

test(
	'Sync and async functions return the same result',
	taraskSync,
	cases.async
);

test(
	'HtmlOptions',
	([text, html]) => taraskSync(text, { html }),
	cases.htmlOptions
);

test('i -> j', ([text, j, abc]) => taraskSync(text, { j, abc }), cases.itoj);

test(
	'greek th',
	(text) => taraskSync(text, { abc: ALPHABET.GREEK }),
	cases.greek
);

print('test', `${summary.passed} passed, ${summary.failed} failed`, '35');

if (process.argv.includes('--benchmark')) {
	const { readFile } = await import('node:fs/promises');

	const text = await readFile('test/large-text.txt', 'utf8');
	benchmark('Taraskevization', () => {
		taraskSync(text, { nonHtml: true });
	});
}

process.exit(summary.failed ? 1 : 0);
