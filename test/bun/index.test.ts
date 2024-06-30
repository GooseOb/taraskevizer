// @ts-nocheck

import { getLabel, testOnCases, testOnCasesAsync } from './lib';
import { tarask, dicts, TaraskConfig, pipelines } from '../../src';
import * as cases from '../cases';
import * as path from 'path';

const objectLabel = getLabel('j');

testOnCases(
	'\x1b[31mTaraskevization',
	(text) => tarask(text, pipelines.plainText),
	cases.taraskevization.change
);

testOnCases(
	'\x1b[31mTaraskevization:no-change',
	(text) => tarask(text, pipelines.plainText),
	cases.taraskevization.noChange
);

testOnCases(
	'\x1b[31mTaraskevization:g-words',
	(text) => tarask(text, pipelines.plainText),
	cases.taraskevization.gwords
);

testOnCases(
	'\x1b[33mHtmlOptions',
	([text, html]) => tarask(text, pipelines.html, new TaraskConfig({ html })),
	cases.htmlOptions,
	objectLabel
);

testOnCases(
	'\x1b[34mNonHtmlOptions',
	([text, nonHtml]) =>
		tarask(text, pipelines.plainText, new TaraskConfig({ nonHtml })),
	cases.nonHtmlOptions,
	objectLabel
);

testOnCases(
	'\x1b[36mi -> j',
	([text, j, abc]) =>
		tarask(
			text,
			pipelines.plainText,
			new TaraskConfig({ general: { j, abc } })
		),
	cases.itoj,
	objectLabel
);

testOnCases(
	'\x1b[31mMultiline',
	(text) => tarask(text, pipelines.plainText),
	cases.multiline.nonHtml,
	objectLabel
);

testOnCases(
	'\x1b[31mMultiline:html',
	(text) => tarask(text, pipelines.html),
	cases.multiline.html,
	objectLabel
);

if (!process.env.NOCLI) {
	const root = path.resolve(import.meta.dir, '..', '..');
	const bunBinArr = [
		'bun',
		path.resolve(
			root,
			(await Bun.file(path.resolve(root, 'package.json')).json()).bin.tarask
		),
	];

	testOnCasesAsync(
		'\x1b[32mCLI',
		(options) => {
			const { stdout, stderr } = Bun.spawn(bunBinArr.concat(options));

			return Bun.readableStreamToText(stderr || stdout).then((text) =>
				text.trim()
			);
		},
		cases.cli,
		objectLabel
	);
}

const latinCfg = new TaraskConfig({
	general: { abc: dicts.alphabets.latin },
});
testOnCases(
	'\x1b[33mLatin',
	(text) => tarask(text, pipelines.plainText, latinCfg),
	cases.latin.general
);

const latinJiCfg = new TaraskConfig({
	general: { abc: dicts.alphabets.latinJi },
});
testOnCases(
	'\x1b[33mLatin:ji',
	(text) => tarask(text, pipelines.plainText, latinJiCfg),
	cases.latin.ji
);

const arabicCfg = new TaraskConfig({
	general: { abc: dicts.alphabets.arabic },
});

testOnCases(
	'\x1b[33mArabic',
	(text) => tarask(text, pipelines.plainText, arabicCfg),
	cases.arabic
);

testOnCases(
	'\x1b[31mSpecialConstructions',
	(text) => tarask(text, pipelines.plainText),
	cases.specialSyntax.general
);

testOnCases(
	'\x1b[31mSpecialConstructions',
	(text) => tarask(text, pipelines.plainText, latinCfg),
	cases.specialSyntax.latin
);

testOnCases(
	'\x1b[31mCaseRestoring:escape-caps',
	(text) => tarask(text, pipelines.plainText),
	cases.caseRestoring.escCap
);

const noEscCapCfg = new TaraskConfig({
	general: { doEscapeCapitalized: false },
});

testOnCases(
	'\x1b[31mCaseRestoring:no-escape-caps',
	(text) => tarask(text, pipelines.plainText, noEscCapCfg),
	cases.caseRestoring.noEscCap
);

testOnCases(
	'\x1b[31mAlphabetConversion:latin',
	(text) => tarask(text, pipelines.abcOnly, latinCfg),
	cases.alphabetConversion.latin
);

testOnCases(
	'\x1b[31mAlphabetConversion:latin-ji',
	(text) => tarask(text, pipelines.abcOnly, latinJiCfg),
	cases.alphabetConversion.latinJi
);

testOnCases(
	'\x1b[31mAlphabetConversion:arabic',
	(text) => tarask(text, pipelines.abcOnly, arabicCfg),
	cases.alphabetConversion.arabic
);

testOnCases(
	'\x1b[31mPhonetization',
	(text) => tarask(text, pipelines.phonetic),
	cases.phonetization
);

// add a new case here
