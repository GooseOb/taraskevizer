import { startTestProcess } from './lib';
import { pipelines, TaraskConfig, dicts, htmlConfigOptions } from '../src';
import * as cases from './cases';
import * as path from 'path';
import { fileURLToPath } from 'url';
import { readFile } from 'node:fs/promises';
import { highlightDiff } from '@/lib';
import { spawn } from 'node:child_process';

const { tarask, alphabetic, phonetic } = pipelines;

const { endTestProcess, test, testAsync } = startTestProcess({ long: false });

test('Taraskevization', (text) => tarask(text), cases.taraskevization.change);

test(
	'Taraskevization:no-change',
	(text) => tarask(text),
	cases.taraskevization.noChange
);

test(
	'Taraskevization:g-words',
	(text) => tarask(text),
	cases.taraskevization.gwords
);

test(
	'HtmlOptions',
	([text, cfg]) => tarask(text, { ...htmlConfigOptions, ...cfg }),
	cases.htmlOptions
);

test(
	'NonHtmlOptions',
	([text, cfg]) => tarask(text, cfg),
	cases.nonHtmlOptions
);

test('i to j', ([text, j, abc]) => tarask(text, { j, abc }), cases.itoj);

test('Multiline', (text) => tarask(text), cases.multiline.nonHtml);

const htmlConfig = new TaraskConfig(htmlConfigOptions);

test(
	'Multiline:html',
	(text) => tarask(text, htmlConfig),
	cases.multiline.html
);

if (process.env.CLI !== '0') {
	let root = path.resolve(fileURLToPath(import.meta.url), '..', '..');
	if (!process.argv[0].endsWith('bun')) root = path.join(root, '..');
	const pathToBin = path.resolve(
		root,
		JSON.parse(await readFile(path.resolve(root, 'package.json'), 'utf-8')).bin
			.tarask
	);
	await testAsync(
		'CLI',
		(options) =>
			new Promise((resolve, reject) => {
				const child = spawn('bun', [pathToBin, ...options]);

				let stdout = '';
				let stderr = '';

				child.stdout.on('data', (data) => {
					stdout += data.toString();
				});

				child.stderr.on('data', (data) => {
					stderr += data.toString();
				});

				child.on('close', (code) => {
					if (code === 0) {
						resolve(stdout.trim());
					} else {
						reject(
							new Error(`Process exited with code ${code}: ${stderr.trim()}`)
						);
					}
				});

				child.on('error', (err) => {
					reject(err);
				});
			}),
		cases.cli
	);
}

const latinCfg = new TaraskConfig({
	abc: dicts.alphabets.latin,
});
test('Latin', (text) => tarask(text, latinCfg), cases.latin.general);

const latinJiCfg = new TaraskConfig({
	abc: dicts.alphabets.latinJi,
});

test('Latin:ji', (text) => tarask(text, latinJiCfg), cases.latin.ji);

const arabicCfg = new TaraskConfig({
	abc: dicts.alphabets.arabic,
});

test('Arabic', (text) => tarask(text, arabicCfg), cases.arabic);

test(
	'SpecialConstructions',
	(text) => tarask(text),
	cases.specialSyntax.general
);

test(
	'SpecialConstructions:latin',
	(text) => tarask(text, latinCfg),
	cases.specialSyntax.latin
);

test(
	'CaseRestoring:escape-caps',
	(text) => tarask(text),
	cases.caseRestoring.escCap
);

const noEscCapCfg = new TaraskConfig({
	doEscapeCapitalized: false,
});

test(
	'CaseRestoring:no-escape-caps',
	(text) => tarask(text, noEscCapCfg),
	cases.caseRestoring.noEscCap
);

test(
	'AlphabetConversion:latin',
	(text) => alphabetic(text, latinCfg),
	cases.alphabetConversion.latin
);

test(
	'AlphabetConversion:latin-ji',
	(text) => alphabetic(text, latinJiCfg),
	cases.alphabetConversion.latinJi
);

test(
	'AlphabetConversion:arabic',
	(text) => alphabetic(text, arabicCfg),
	cases.alphabetConversion.arabic
);

const phoneticCfg = new TaraskConfig({
	j: 'always',
});

test(
	'Phonetization',
	(text) => phonetic(text, phoneticCfg),
	cases.phonetization
);

test(
	'Highlighting',
	(input) => {
		// @ts-expect-error fdnsj jnfsdn
		const text: [string] = [input[1] || tarask(input[0])];
		highlightDiff(text, [input[0]], true, (text) => `[${text}]`);
		return text[0];
	},
	cases.highlighting
);

// add a new case here

process.exit(endTestProcess());
