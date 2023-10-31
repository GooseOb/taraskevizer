import { benchmark, print, startTestProcess } from './lib';
import { tarask, ALPHABET } from '../src';
import * as cases from './cases';

const { endTestProcess, test } = startTestProcess();

test('Taraskevization', tarask, cases.taraskevization);

test(
	'HtmlOptions',
	([text, html]) => tarask(text, { html }),
	cases.htmlOptions
);
test(
	'NonHtmlOptions',
	([text, nonHtml]) => tarask(text, { nonHtml }),
	cases.nonHtmlOptions
);

test('i to j', ([text, j, abc]) => tarask(text, { j, abc }), cases.itoj);

test('Greek th', (text) => tarask(text, { abc: ALPHABET.GREEK }), cases.greek);

test('Multiline', ([text, options]) => tarask(text, options), cases.multiline);

// add a new case here

const code = endTestProcess();

if (process.argv.includes('--benchmark')) {
	const {
		existsSync,
		promises: { readFile },
	} = await import('node:fs');
	const path = 'test/large-text.txt';

	if (!existsSync(path)) {
		print('benchmark', path + ': no such a file', '36');
		process.exit(1);
	}
	const text = await readFile(path, 'utf8');
	benchmark('Taraskevization', () => {
		tarask(text, { nonHtml: true });
	});
}

process.exit(code);
