import { startTestProcess } from './lib';
import {
	tarask,
	pipelines,
	TaraskConfig,
	dicts,
	htmlConfigOptions,
} from '../src';
import * as cases from './cases';
import * as path from 'path';
import { spawnSync } from 'child_process';
import { fileURLToPath } from 'url';
import { readFile } from 'node:fs/promises';

const { tar, abc } = pipelines;

const { endTestProcess, test } = startTestProcess({ long: false });

test(
	'Taraskevization',
	(text) => tarask(text, tar),
	cases.taraskevization.change
);

test(
	'Taraskevization:no-change',
	(text) => tarask(text, tar),
	cases.taraskevization.noChange
);

test(
	'Taraskevization:g-words',
	(text) => tarask(text, tar),
	cases.taraskevization.gwords
);

test(
	'HtmlOptions',
	([text, cfg]) =>
		tarask(text, tar, new TaraskConfig({ ...htmlConfigOptions, ...cfg })),
	cases.htmlOptions
);

test(
	'NonHtmlOptions',
	([text, cfg]) => tarask(text, tar, new TaraskConfig(cfg)),
	cases.nonHtmlOptions
);

test(
	'i to j',
	([text, j, abc]) => tarask(text, tar, new TaraskConfig({ j, abc })),
	cases.itoj
);

test('Multiline', (text) => tarask(text, tar), cases.multiline.nonHtml);
test(
	'Multiline:html',
	(text) => tarask(text, tar, new TaraskConfig(htmlConfigOptions)),
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

const latinCfg = new TaraskConfig({
	abc: dicts.alphabets.latin,
});
test('Latin', (text) => tarask(text, tar, latinCfg), cases.latin.general);

const latinJiCfg = new TaraskConfig({
	abc: dicts.alphabets.latinJi,
});

test('Latin:ji', (text) => tarask(text, tar, latinJiCfg), cases.latin.ji);

const arabicCfg = new TaraskConfig({
	abc: dicts.alphabets.arabic,
});

test('Arabic', (text) => tarask(text, tar, arabicCfg), cases.arabic);

test(
	'SpecialConstructions',
	(text) => tarask(text, tar),
	cases.specialSyntax.general
);

test(
	'SpecialConstructions:latin',
	(text) => tarask(text, tar, latinCfg),
	cases.specialSyntax.latin
);

test(
	'CaseRestoring:escape-caps',
	(text) => tarask(text, tar),
	cases.caseRestoring.escCap
);

const noEscCapCfg = new TaraskConfig({
	doEscapeCapitalized: false,
});

test(
	'CaseRestoring:no-escape-caps',
	(text) => tarask(text, tar, noEscCapCfg),
	cases.caseRestoring.noEscCap
);

test(
	'AlphabetConversion:latin',
	(text) => tarask(text, abc, latinCfg),
	cases.alphabetConversion.latin
);

test(
	'AlphabetConversion:latin-ji',
	(text) => tarask(text, abc, latinJiCfg),
	cases.alphabetConversion.latinJi
);

test(
	'AlphabetConversion:arabic',
	(text) => tarask(text, abc, arabicCfg),
	cases.alphabetConversion.arabic
);

const phoneticCfg = new TaraskConfig({
	j: 'always',
});

test(
	'Phonetization',
	(text) => tarask(text, pipelines.phonetic, phoneticCfg),
	cases.phonetization
);

// add a new case here

process.exit(endTestProcess());
