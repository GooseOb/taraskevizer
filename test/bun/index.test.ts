import { getLabel, testOnCases } from './lib';
import { ALPHABET, taraskSync } from '../../src';
import * as cases from '../cases';

testOnCases('\x1b[31mTaraskevization', taraskSync, cases.taraskevization);

testOnCases(
	'\x1b[32mSync and async functions return the same result',
	taraskSync,
	cases.async
);

testOnCases(
	'\x1b[33mHtmlOptions',
	([text, html]) => taraskSync(text, { html }),
	cases.htmlOptions,
	getLabel('j')
);

testOnCases(
	'\x1b[34mi -> j',
	([text, j, abc]) => taraskSync(text, { j, abc }),
	cases.itoj,
	getLabel('j')
);

testOnCases(
	'\x1b[36mgreek th',
	(text) => taraskSync(text, { abc: ALPHABET.GREEK }),
	cases.greek
);
