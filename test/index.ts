import { benchmark, print, startTestProcess } from './lib';
import { Taraskevizer, ALPHABET } from '../src';
import * as cases from './cases';
import * as path from 'path';
import { spawnSync } from 'child_process';
import { fileURLToPath } from 'url';
import { readFile } from 'node:fs/promises';

const { endTestProcess, test } = startTestProcess({ long: false });

const taraskevizer = new Taraskevizer();
test(
	'Taraskevization:no-change',
	(text) => taraskevizer.convert(text),
	cases.taraskevization.change
);

test(
	'Taraskevization',
	(text) => taraskevizer.convert(text),
	cases.taraskevization.noChange
);

test(
	'HtmlOptions',
	([text, html]) => new Taraskevizer({ html }).convertToHtml(text),
	cases.htmlOptions
);

test(
	'NonHtmlOptions',
	([text, nonHtml]) => new Taraskevizer({ nonHtml }).convert(text),
	cases.nonHtmlOptions
);

test(
	'i to j',
	([text, j, abc]) => new Taraskevizer({ general: { j, abc } }).convert(text),
	cases.itoj
);

test(
	'Multiline',
	(text) => taraskevizer.convert(text),
	cases.multiline.nonHtml
);
test(
	'Multiline:html',
	(text) => taraskevizer.convertToHtml(text),
	cases.multiline.html
);

if (!process.env.NOCLI) {
	const root = path.resolve(fileURLToPath(import.meta.url), '..', '..', '..');
	const pathToBin = path.resolve(
		root,
		JSON.parse(await readFile(path.resolve(root, 'package.json'), 'utf-8')).bin
			.tarask
	);
	test(
		'CLI',
		(options) => {
			const { stdout, stderr } = spawnSync('node', [pathToBin, ...options], {
				encoding: 'utf-8',
			});

			if (stderr) process.stderr.write(stderr);

			return stdout.trim();
		},
		cases.cli
	);
}

const taraskevizerLatin = new Taraskevizer({
	general: { abc: ALPHABET.LATIN },
});
test('Latin', (text) => taraskevizerLatin.convert(text), cases.latin);

test(
	'SpecialConstructions',
	(text) => taraskevizer.convert(text),
	cases.specialConstructions
);

// add a new case here

const code = endTestProcess();

if (process.argv.includes('--benchmark')) {
	const { existsSync } = await import('node:fs');
	const path = 'test/large-text.txt';

	if (!existsSync(path)) {
		print('benchmark', path + ': no such a file', '36');
		process.exit(1);
	}
	const text = await readFile(path, 'utf8');
	benchmark('Taraskevization', () => {
		taraskevizer.convert(text);
	});
}

process.exit(code);
