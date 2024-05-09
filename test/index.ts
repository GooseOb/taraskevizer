import { startTestProcess } from './lib';
import { Taraskevizer, ALPHABET } from '../src';
import * as cases from './cases';
import * as path from 'path';
import { spawnSync } from 'child_process';
import { fileURLToPath } from 'url';
import { readFile } from 'node:fs/promises';

const { endTestProcess, test } = startTestProcess({ long: false });

const taraskevizer = new Taraskevizer();
test(
	'Taraskevization',
	(text) => taraskevizer.convert(text),
	cases.taraskevization.change
);

test(
	'Taraskevization:no-change',
	(text) => taraskevizer.convert(text),
	cases.taraskevization.noChange
);

test(
	'Taraskevization:g-words',
	(text) => taraskevizer.convert(text),
	cases.taraskevization.gwords
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
			const { stdout, stderr } = spawnSync('bun', [pathToBin, ...options], {
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
test('Latin', (text) => taraskevizerLatin.convert(text), cases.latin.general);

const taraskevizerLatinJi = new Taraskevizer({
	general: { abc: ALPHABET.LATIN_JI },
});

test('Latin:ji', (text) => taraskevizerLatinJi.convert(text), cases.latin.ji);

test(
	'SpecialConstructions',
	(text) => taraskevizer.convert(text),
	cases.specialConstructions.general
);

test(
	'SpecialConstructions:latin',
	(text) => taraskevizerLatin.convert(text),
	cases.specialConstructions.latin
);

test(
	'CaseRestoring:escape-caps',
	(text) => taraskevizer.convert(text),
	cases.caseRestoring.escCap
);

const taraskevizerNoEscCap = new Taraskevizer({
	general: { doEscapeCapitalized: false },
});

test(
	'CaseRestoring:no-escape-caps',
	(text) => taraskevizerNoEscCap.convert(text),
	cases.caseRestoring.noEscCap
);

test(
	'AlphabetConversion:latin-no-ji',
	(text) => taraskevizerLatin.convertAlphabetOnly(text),
	cases.alphabetConversion.latin
);

test(
	'AlphabetConversion:latin-ji',
	(text) => taraskevizerLatinJi.convertAlphabetOnly(text),
	cases.alphabetConversion.latinJi
);

// add a new case here

process.exit(endTestProcess());
