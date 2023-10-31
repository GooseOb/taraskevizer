import { benchmark, print, startTestProcess } from './lib';
import { tarask, ALPHABET, taraskToHtml } from '../src';
import * as cases from './cases';
import * as path from 'path';
import { spawnSync } from 'child_process';
import { fileURLToPath } from 'url';

const { endTestProcess, test } = startTestProcess({ long: false });

test('Taraskevization', tarask, cases.taraskevization);

test(
	'HtmlOptions',
	([text, html]) => taraskToHtml(text, {}, html),
	cases.htmlOptions
);
test(
	'NonHtmlOptions',
	([text, nonHtml]) => tarask(text, {}, nonHtml),
	cases.nonHtmlOptions
);

test('i to j', ([text, j, abc]) => tarask(text, { j, abc }), cases.itoj);

test('Greek th', (text) => tarask(text, { abc: ALPHABET.GREEK }), cases.greek);

test('Multiline', tarask, cases.multiline.nonHtml);
test('Multiline:html', taraskToHtml, cases.multiline.html);

const pathToBin = path.resolve(
	fileURLToPath(import.meta.url).replace(/taraskevizer[\\/].+/, 'taraskevizer'),
	'bin',
	'index.js'
);
test(
	'CLI',
	(options) => {
		const { stdout, stderr } = spawnSync('node', [pathToBin, ...options], {
			encoding: 'utf-8',
		});

		if (stderr) {
			process.stderr.write(stderr);
		}

		return stdout.trim();
	},
	cases.cli
);

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
		tarask(text);
	});
}

process.exit(code);
