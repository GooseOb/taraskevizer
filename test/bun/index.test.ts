import { getLabel, testOnCases } from './lib';
import { ALPHABET, tarask } from '../../src';
import * as cases from '../cases';

testOnCases('\x1b[31mTaraskevization', tarask, cases.taraskevization);

testOnCases(
	'\x1b[33mHtmlOptions',
	([text, html]) => tarask(text, { html }),
	cases.htmlOptions,
	getLabel('j')
);

testOnCases(
	'\x1b[34mNonHtmlOptions',
	([text, nonHtml]) => tarask(text, { nonHtml }),
	cases.nonHtmlOptions,
	getLabel('j')
);

testOnCases(
	'\x1b[36mi -> j',
	([text, j, abc]) => tarask(text, { j, abc }),
	cases.itoj,
	getLabel('j')
);

testOnCases(
	'\x1b[37mgreek th',
	(text) => tarask(text, { abc: ALPHABET.GREEK }),
	cases.greek
);
