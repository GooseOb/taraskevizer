import { getLabel, testOnCases, testOnCasesAsync } from './lib';
import {
	tarask,
	dicts,
	TaraskConfig,
	pipelines,
	htmlConfigOptions,
} from '../../src';
import * as cases from '../cases';
import * as path from 'path';

const objectLabel = getLabel('j');

testOnCases(
	'\x1b[31mTaraskevization',
	(text) => tarask(text, pipelines.tar),
	cases.taraskevization.change
);

testOnCases(
	'\x1b[31mTaraskevization:no-change',
	(text) => tarask(text, pipelines.tar),
	cases.taraskevization.noChange
);

testOnCases(
	'\x1b[31mTaraskevization:g-words',
	(text) => tarask(text, pipelines.tar),
	cases.taraskevization.gwords
);

testOnCases(
	'\x1b[33mHtmlOptions',
	([text, cfg]) =>
		tarask(
			text,
			pipelines.tar,
			new TaraskConfig({ ...htmlConfigOptions, ...cfg })
		),
	cases.htmlOptions,
	objectLabel
);

testOnCases(
	'\x1b[34mNonHtmlOptions',
	([text, cfg]) => tarask(text, pipelines.tar, new TaraskConfig(cfg)),
	cases.nonHtmlOptions,
	objectLabel
);

testOnCases(
	'\x1b[36mi -> j',
	([text, j, abc]) => tarask(text, pipelines.tar, new TaraskConfig({ j, abc })),
	cases.itoj,
	objectLabel
);

testOnCases(
	'\x1b[31mMultiline',
	(text) => tarask(text, pipelines.tar),
	cases.multiline.nonHtml,
	objectLabel
);

testOnCases(
	'\x1b[31mMultiline:html',
	(text) => tarask(text, pipelines.tar, new TaraskConfig(htmlConfigOptions)),
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
	abc: dicts.alphabets.latin,
});
testOnCases(
	'\x1b[33mLatin',
	(text) => tarask(text, pipelines.tar, latinCfg),
	cases.latin.general
);

const latinJiCfg = new TaraskConfig({
	abc: dicts.alphabets.latinJi,
});
testOnCases(
	'\x1b[33mLatin:ji',
	(text) => tarask(text, pipelines.tar, latinJiCfg),
	cases.latin.ji
);

const arabicCfg = new TaraskConfig({
	abc: dicts.alphabets.arabic,
});

testOnCases(
	'\x1b[33mArabic',
	(text) => tarask(text, pipelines.tar, arabicCfg),
	cases.arabic
);

testOnCases(
	'\x1b[31mSpecialConstructions',
	(text) => tarask(text, pipelines.tar),
	cases.specialSyntax.general
);

testOnCases(
	'\x1b[31mSpecialConstructions',
	(text) => tarask(text, pipelines.tar, latinCfg),
	cases.specialSyntax.latin
);

testOnCases(
	'\x1b[31mCaseRestoring:escape-caps',
	(text) => tarask(text, pipelines.tar),
	cases.caseRestoring.escCap
);

const noEscCapCfg = new TaraskConfig({
	doEscapeCapitalized: false,
});

testOnCases(
	'\x1b[31mCaseRestoring:no-escape-caps',
	(text) => tarask(text, pipelines.tar, noEscCapCfg),
	cases.caseRestoring.noEscCap
);

testOnCases(
	'\x1b[31mAlphabetConversion:latin',
	(text) => tarask(text, pipelines.abc, latinCfg),
	cases.alphabetConversion.latin
);

testOnCases(
	'\x1b[31mAlphabetConversion:latin-ji',
	(text) => tarask(text, pipelines.abc, latinJiCfg),
	cases.alphabetConversion.latinJi
);

testOnCases(
	'\x1b[31mAlphabetConversion:arabic',
	(text) => tarask(text, pipelines.abc, arabicCfg),
	cases.alphabetConversion.arabic
);

testOnCases(
	'\x1b[31mPhonetization',
	(text) => tarask(text, pipelines.phonetic),
	cases.phonetization
);

// add a new case here
