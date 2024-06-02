import { getLabel, testOnCases, testOnCasesAsync } from './lib';
import {
	tarask,
	plainTextPipeline,
	dicts,
	TaraskConfig,
	htmlPipeline,
	abcOnlyPipeline,
} from '../../src';
import * as cases from '../cases';
import * as path from 'path';

const objectLabel = getLabel('j');

testOnCases(
	'\x1b[31mTaraskevization',
	(text) => tarask(text, plainTextPipeline),
	cases.taraskevization.change
);

testOnCases(
	'\x1b[31mTaraskevization:no-change',
	(text) => tarask(text, plainTextPipeline),
	cases.taraskevization.noChange
);

testOnCases(
	'\x1b[31mTaraskevization:g-words',
	(text) => tarask(text, plainTextPipeline),
	cases.taraskevization.gwords
);

testOnCases(
	'\x1b[33mHtmlOptions',
	([text, html]) => tarask(text, htmlPipeline, new TaraskConfig({ html })),
	cases.htmlOptions,
	objectLabel
);

testOnCases(
	'\x1b[34mNonHtmlOptions',
	([text, nonHtml]) =>
		tarask(text, plainTextPipeline, new TaraskConfig({ nonHtml })),
	cases.nonHtmlOptions,
	objectLabel
);

testOnCases(
	'\x1b[36mi -> j',
	([text, j, abc]) =>
		tarask(text, plainTextPipeline, new TaraskConfig({ general: { j, abc } })),
	cases.itoj,
	objectLabel
);

testOnCases(
	'\x1b[31mMultiline',
	(text) => tarask(text, plainTextPipeline),
	cases.multiline.nonHtml,
	objectLabel
);

testOnCases(
	'\x1b[31mMultiline:html',
	(text) => tarask(text, htmlPipeline),
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
	(text) => tarask(text, plainTextPipeline, latinCfg),
	cases.latin.general
);

const latinJiCfg = new TaraskConfig({
	general: { abc: dicts.alphabets.latinJi },
});
testOnCases(
	'\x1b[33mLatin:ji',
	(text) => tarask(text, plainTextPipeline, latinJiCfg),
	cases.latin.ji
);

testOnCases(
	'\x1b[31mSpecialConstructions',
	(text) => tarask(text, plainTextPipeline),
	cases.specialSyntax.general
);

testOnCases(
	'\x1b[31mSpecialConstructions',
	(text) => tarask(text, plainTextPipeline, latinCfg),
	cases.specialSyntax.latin
);

testOnCases(
	'\x1b[31mCaseRestoring:escape-caps',
	(text) => tarask(text, plainTextPipeline),
	cases.caseRestoring.escCap
);

const noEscCapCfg = new TaraskConfig({
	general: { doEscapeCapitalized: false },
});

testOnCases(
	'\x1b[31mCaseRestoring:no-escape-caps',
	(text) => tarask(text, plainTextPipeline, noEscCapCfg),
	cases.caseRestoring.noEscCap
);

testOnCases(
	'\x1b[31mAlphabetConversion:latin-no-ji',
	(text) => tarask(text, abcOnlyPipeline, latinCfg),
	cases.alphabetConversion.latin
);

testOnCases(
	'\x1b[31mAlphabetConversion:latin-ji',
	(text) => tarask(text, abcOnlyPipeline, latinJiCfg),
	cases.alphabetConversion.latinJi
);

// add a new case here
