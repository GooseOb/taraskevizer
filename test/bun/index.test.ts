import { getLabel, testOnCases, testOnCasesAsync } from './lib';
import { ALPHABET, tarask, taraskToHtml } from '../../src';
import * as cases from '../cases';
import * as path from 'path';

const objectLabel = getLabel('j');

testOnCases('\x1b[31mTaraskevization', tarask, cases.taraskevization);

testOnCases(
	'\x1b[33mHtmlOptions',
	([text, html]) => taraskToHtml(text, {}, html),
	cases.htmlOptions,
	objectLabel
);

testOnCases(
	'\x1b[34mNonHtmlOptions',
	([text, nonHtml]) => tarask(text, {}, nonHtml),
	cases.nonHtmlOptions,
	objectLabel
);

testOnCases(
	'\x1b[36mi -> j',
	([text, j, abc]) => tarask(text, { j, abc }),
	cases.itoj,
	objectLabel
);

testOnCases(
	'\x1b[37mGreek th',
	(text) => tarask(text, { abc: ALPHABET.GREEK }),
	cases.greek
);

testOnCases('\x1b[31mMultiline', tarask, cases.multiline.nonHtml, objectLabel);

testOnCases(
	'\x1b[31mMultiline:html',
	taraskToHtml,
	cases.multiline.html,
	objectLabel
);

if (process.env.NO_CLI !== 'true') {
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

			if (stderr) {
				process.stderr.write(stderr);
			}

			return Bun.readableStreamToText(stdout).then((text) => text.trim());
		},
		cases.cli,
		objectLabel
	);
}

testOnCases(
	'\x1b[33mLatin',
	(text) => tarask(text, { abc: ALPHABET.LATIN }),
	cases.latin
);

// add a new case here
