import { startTestProcess } from './lib';
import {
	tarask,
	htmlPipeline,
	abcOnlyPipeline,
	plainTextPipeline,
	TaraskConfig,
	dicts,
} from '../src';
import * as cases from './cases';
import * as path from 'path';
import { spawnSync } from 'child_process';
import { fileURLToPath } from 'url';
import { readFile } from 'node:fs/promises';

const { endTestProcess, test } = startTestProcess({ long: false });

test(
	'Taraskevization',
	(text) => tarask(text, plainTextPipeline),
	cases.taraskevization.change
);

test(
	'Taraskevization:no-change',
	(text) => tarask(text, plainTextPipeline),
	cases.taraskevization.noChange
);

test(
	'Taraskevization:g-words',
	(text) => tarask(text, plainTextPipeline),
	cases.taraskevization.gwords
);

test(
	'HtmlOptions',
	([text, html]) => tarask(text, htmlPipeline, new TaraskConfig({ html })),
	cases.htmlOptions
);

test(
	'NonHtmlOptions',
	([text, nonHtml]) =>
		tarask(text, plainTextPipeline, new TaraskConfig({ nonHtml })),
	cases.nonHtmlOptions
);

test(
	'i to j',
	([text, j, abc]) =>
		tarask(text, plainTextPipeline, new TaraskConfig({ general: { j, abc } })),
	cases.itoj
);

test(
	'Multiline',
	(text) => tarask(text, plainTextPipeline),
	cases.multiline.nonHtml
);
test(
	'Multiline:html',
	(text) => tarask(text, htmlPipeline),
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
	general: { abc: dicts.alphabets.latin },
});
test(
	'Latin',
	(text) => tarask(text, plainTextPipeline, latinCfg),
	cases.latin.general
);

const latinJiCfg = new TaraskConfig({
	general: { abc: dicts.alphabets.latinJi },
});

test(
	'Latin:ji',
	(text) => tarask(text, plainTextPipeline, latinJiCfg),
	cases.latin.ji
);

const arabicCfg = new TaraskConfig({
	general: { abc: dicts.alphabets.arabic },
});

test(
	'Arabic',
	(text) => tarask(text, plainTextPipeline, arabicCfg),
	cases.arabic
);

test(
	'SpecialConstructions',
	(text) => tarask(text, plainTextPipeline),
	cases.specialSyntax.general
);

test(
	'SpecialConstructions:latin',
	(text) => tarask(text, plainTextPipeline, latinCfg),
	cases.specialSyntax.latin
);

test(
	'CaseRestoring:escape-caps',
	(text) => tarask(text, plainTextPipeline),
	cases.caseRestoring.escCap
);

const noEscCapCfg = new TaraskConfig({
	general: { doEscapeCapitalized: false },
});

test(
	'CaseRestoring:no-escape-caps',
	(text) => tarask(text, plainTextPipeline, noEscCapCfg),
	cases.caseRestoring.noEscCap
);

test(
	'AlphabetConversion:latin',
	(text) => tarask(text, abcOnlyPipeline, latinCfg),
	cases.alphabetConversion.latin
);

test(
	'AlphabetConversion:latin-ji',
	(text) => tarask(text, abcOnlyPipeline, latinJiCfg),
	cases.alphabetConversion.latinJi
);

test(
	'AlphabetConversion:arabic',
	(text) => tarask(text, abcOnlyPipeline, arabicCfg),
	cases.alphabetConversion.arabic
);

// add a new case here

process.exit(endTestProcess());
