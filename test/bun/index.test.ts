import { getLabel, testOnCases, testOnCasesAsync } from './lib';
import { Taraskevizer, ALPHABET } from '../../src';
import * as cases from '../cases';
import * as path from 'path';

const objectLabel = getLabel('j');

const taraskevizer = new Taraskevizer();
testOnCases(
	'\x1b[31mTaraskevization',
	(text) => taraskevizer.convert(text),
	cases.taraskevization.change
);

testOnCases(
	'\x1b[31mTaraskevization:no-change',
	(text) => taraskevizer.convert(text),
	cases.taraskevization.noChange
);

testOnCases(
	'\x1b[31mTaraskevization:g-words',
	(text) => taraskevizer.convert(text),
	cases.taraskevization.gwords
);

testOnCases(
	'\x1b[33mHtmlOptions',
	([text, html]) => new Taraskevizer({ html }).convertToHtml(text),
	cases.htmlOptions,
	objectLabel
);

testOnCases(
	'\x1b[34mNonHtmlOptions',
	([text, nonHtml]) => new Taraskevizer({ nonHtml }).convert(text),
	cases.nonHtmlOptions,
	objectLabel
);

testOnCases(
	'\x1b[36mi -> j',
	([text, j, abc]) => new Taraskevizer({ general: { j, abc } }).convert(text),
	cases.itoj,
	objectLabel
);

testOnCases(
	'\x1b[31mMultiline',
	(text) => taraskevizer.convert(text),
	cases.multiline.nonHtml,
	objectLabel
);

testOnCases(
	'\x1b[31mMultiline:html',
	(text) => taraskevizer.convertToHtml(text),
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

const taraskevizerLatin = new Taraskevizer({
	general: { abc: ALPHABET.LATIN },
});
testOnCases(
	'\x1b[33mLatin',
	(text) => taraskevizerLatin.convert(text),
	cases.latin.general
);

const taraskevizerLatinJi = new Taraskevizer({
	general: { abc: ALPHABET.LATIN_JI },
});
testOnCases(
	'\x1b[33mLatin:ji',
	(text) => taraskevizerLatinJi.convert(text),
	cases.latin.ji
);

testOnCases(
	'\x1b[31mSpecialConstructions',
	(text) => taraskevizer.convert(text),
	cases.specialSyntax.general
);

testOnCases(
	'\x1b[31mSpecialConstructions',
	(text) => taraskevizerLatin.convert(text),
	cases.specialSyntax.latin
);

testOnCases(
	'\x1b[31mCaseRestoring:escape-caps',
	(text) => taraskevizer.convert(text),
	cases.caseRestoring.escCap
);

const taraskevizerNoEscCap = new Taraskevizer({
	general: { doEscapeCapitalized: false },
});

testOnCases(
	'\x1b[31mCaseRestoring:no-escape-caps',
	(text) => taraskevizerNoEscCap.convert(text),
	cases.caseRestoring.noEscCap
);

testOnCases(
	'\x1b[31mAlphabetConversion:latin-no-ji',
	(text) => taraskevizerLatin.convertAlphabetOnly(text),
	cases.alphabetConversion.latin
);

testOnCases(
	'\x1b[31mAlphabetConversion:latin-ji',
	(text) => taraskevizerLatinJi.convertAlphabetOnly(text),
	cases.alphabetConversion.latinJi
);

// add a new case here
