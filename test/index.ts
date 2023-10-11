import { benchmark, startTestProcess } from './lib';
import { tarask, ALPHABET } from '../src';
import * as cases from './cases';

const { endTestProcess, test } = startTestProcess();

test('Taraskevization', tarask, cases.taraskevization);

test(
	'HtmlOptions',
	([text, html]) => tarask(text, { html }),
	cases.htmlOptions
);

test('i -> j', ([text, j, abc]) => tarask(text, { j, abc }), cases.itoj);

test('greek th', (text) => tarask(text, { abc: ALPHABET.GREEK }), cases.greek);

const code = endTestProcess();

if (process.argv.includes('--benchmark')) {
	const { readFile } = await import('node:fs/promises');

	const text = await readFile('test/large-text.txt', 'utf8');
	benchmark('Taraskevization', () => {
		tarask(text, { nonHtml: true });
	});
}

process.exit(code);
