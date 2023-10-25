import { getLabel, testOnCases } from './lib';
import { ALPHABET, tarask } from '../../src';
import * as cases from '../cases';

const jLabel = getLabel('j');

testOnCases('\x1b[31mTaraskevization', tarask, cases.taraskevization);

testOnCases(
	'\x1b[33mHtmlOptions',
	([text, html]) => tarask(text, { html }),
	cases.htmlOptions,
	jLabel
);

testOnCases(
	'\x1b[34mNonHtmlOptions',
	([text, nonHtml]) => tarask(text, { nonHtml }),
	cases.nonHtmlOptions,
	jLabel
);

testOnCases(
	'\x1b[36mi -> j',
	([text, j, abc]) => tarask(text, { j, abc }),
	cases.itoj,
	jLabel
);

testOnCases(
	'\x1b[37mGreek th',
	(text) => tarask(text, { abc: ALPHABET.GREEK }),
	cases.greek
);

testOnCases(
	'\x1b[31mMultiline',
	([text, options]) => tarask(text, options),
	cases.multiline,
	jLabel
);

// add a new case here
