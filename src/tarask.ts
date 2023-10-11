import {
	wordlist,
	softers,
	arabLetters,
	latinLetters,
	latinLettersUpperCase,
	gobj,
	greekLetters,
	greekLettersUpperCase,
	thWords,
} from './dict';
import {
	Tarask,
	AlphabetDependentDict,
	TaraskOptions,
	ExtendedDict,
	ToTarask,
	ReplaceWithDict,
} from './types';
import * as debug from './tools.debug';

const isObject = <T extends object>(arg: any): arg is T =>
	typeof arg === 'object';

const isUpperCase = (str: string): boolean => str === str.toUpperCase();

const getLastLetter = (word: string) => {
	for (let i = word.length - 1; i >= 0; i--)
		if (/\p{L}/u.test(word[i])) return word[i];
	throw new Error(`the last letter of the word ${word} not found`);
};

const NOFIX_CHAR = ' \uffff ';
const NOFIX_REGEX = new RegExp(NOFIX_CHAR, 'g');
const OPTIONAL_WORDS_REGEX = /\(.*?\)/g;
const G_REGEX = /[Ґґ]/g;
type G_REGEX_MATCH = 'Ґ' | 'ґ';

export const ALPHABET = { CYRILLIC: 0, LATIN: 1, ARABIC: 2, GREEK: 3 } as const;
export const J = { NEVER: 0, RANDOM: 1, ALWAYS: 2 } as const;
export const VARIATION = { NO: 0, FIRST: 1, ALL: 2 } as const;

const letters: AlphabetDependentDict = {
	[ALPHABET.LATIN]: latinLetters,
	[ALPHABET.ARABIC]: arabLetters,
	[ALPHABET.GREEK]: greekLetters,
};
const lettersUpperCase: AlphabetDependentDict = {
	[ALPHABET.LATIN]: latinLettersUpperCase,
	[ALPHABET.GREEK]: greekLettersUpperCase,
};

type SpecificApplyObj = Record<'F' | 'H' | 'L', (content: string) => string>;

const tagApplications = {
	html: {
		F: (content) => `<tarF>${content}</tarF>`,
		H: (content) => `<tarH>${content}</tarH>`,
	},
	nonHtml: {
		F: (content) => `\x1b[32m${content}\x1b[0m`,
		H: (content) => `\x1b[35m${content}\x1b[0m`,
		L: (content) => `\x1b[35m${content}\x1b[0m`,
	},
} satisfies Record<'html' | 'nonHtml', Partial<SpecificApplyObj>>;

const iaReplacer = ($0: string, $1: string, $2: string) =>
	$2.match(/[аеёіоуыэюя]/g)?.length === 1 ? $1 + 'я' + $2 : $0;

const afterTarask: ExtendedDict = [
	[/ [уў]асьнігл /g, ' уаснігл '],
	[/ сь(?=нід |мі )/g, ' с'],
	[/( б)е(зь? \S+)/g, iaReplacer],
	[/( н)е( \S+)/g, iaReplacer],
	[
		/( (?:б[ея]|пра|цера)?з) і(\S*)/g,
		($0, $1, $2) => (/([ая]ў|ну)$/.test($2) ? $1 + 'ь і' + $2 : $0),
	],
];

export const tarask: Tarask = (text, options = {}) => {
	const { abc = 0, j = 0, html = false, nonHtml = false } = options;
	const isHtmlObject = isObject(html);
	const isNonHtmlObject = isObject(nonHtml);
	// if (isHtmlObject) {
	// } else
	if (isNonHtmlObject) {
		nonHtml.variations ||= 0;
	}
	const apply = html ? tagApplications.html : tagApplications.nonHtml;
	const noFix: string[] = [];

	const LEFT_ANGLE_BRACKET = html ? '&lt;' : '<';

	text = ` ${text.trim()}  `
		.replace(/\ufffd/g, '')
		.replace(/<([,.]?)((?:.|\s)*?)>/g, ($0, $1, $2) => {
			if ($1 === ',') return LEFT_ANGLE_BRACKET + $2 + '>';
			noFix[noFix.length] = $1 === '.' ? $2 : $0;
			return NOFIX_CHAR;
		})
		.replace(/г'(?![еёіюя])/g, 'ґ')
		.replace(/([\n\t])/g, ' $1 ')
		.replace(/ - /g, ' — ')
		.replace(/(\p{P}|\p{S}|\d)/gu, ' $1 ')
		.replace(/ ['`’] (?=\S)/g, 'ʼ')
		.replace(/\(/g, '&#40');

	let splittedOrig: string[], splitted: string[];
	splittedOrig = replaceWithDict(
		replaceWithDict(text, letters[abc]),
		lettersUpperCase[abc]
	).split(' ');

	text = (options.OVERRIDE_toTarask || toTarask)(
		text.toLowerCase(),
		replaceWithDict,
		wordlist,
		softers,
		afterTarask
	);
	if (j) text = replaceIbyJ(text, j === J.ALWAYS);
	if (abc === ALPHABET.GREEK) text = replaceWithDict(text, thWords);
	text = replaceWithDict(text, letters[abc]);

	splitted = text.split(' ');
	if (abc !== ALPHABET.ARABIC) splitted = restoreCase(splitted, splittedOrig);
	if (html || (isNonHtmlObject && nonHtml.nodeColors))
		splitted = toTags(
			splitted,
			splittedOrig,
			abc === ALPHABET.CYRILLIC,
			apply.F
		);

	text = splitted
		.join(' ')
		.replace(/&nbsp;/g, ' ')
		.replace(/ (\p{P}|\p{S}|\d|&#40) /gu, '$1');

	let gReplacer: undefined | string | ((substring: G_REGEX_MATCH) => string);
	if (abc === ALPHABET.CYRILLIC) {
		if (isHtmlObject) {
			gReplacer = html.g ? apply.H('$&') : ($0) => apply.H(gobj[$0]);
		} else if (isNonHtmlObject) {
			if (nonHtml.nodeColors) {
				gReplacer = nonHtml.h ? ($0) => apply.H(gobj[$0]) : apply.H('$&');
			} else if (nonHtml.h) {
				gReplacer = ($0) => gobj[$0];
			}
		}
	}

	if (gReplacer)
		text = text.replace(
			G_REGEX,
			// @ts-ignore
			gReplacer
		);

	if (noFix.length)
		text = text.replace(NOFIX_REGEX, () => noFix.shift() as string);

	return (
		html ? finalizer.html(text) : finalizer.nonHtml(text, nonHtml)
	).trim();
};

const restoreCase = (text: string[], orig: string[]): string[] => {
	for (let i = 0; i < text.length; i++) {
		const word = text[i];
		const oWord = orig[i];
		if (word === oWord) continue;
		if (word === oWord.toLowerCase()) {
			text[i] = oWord;
			continue;
		}
		if (!oWord[0] || !isUpperCase(oWord[0])) continue;
		if (word === 'зь') {
			text[i] = isUpperCase(orig[i + 1]) ? 'ЗЬ' : 'Зь';
		} else if (isUpperCase(getLastLetter(oWord))) {
			text[i] = word.toUpperCase();
		} else {
			text[i] =
				word[0] === '('
					? word.replace(/.*?(?=\))/, ($0) =>
							$0.replace(/[(|]./g, ($0) => $0.toUpperCase())
					  )
					: word[0].toUpperCase() + word.slice(1);
		}
	}

	return text;
};

const toTags = (
	text: string[],
	orig: string[],
	isCyrillic: boolean,
	applyF: (content: string) => string
): string[] => {
	for (let i = 0; i < text.length; i++) {
		const word = text[i];
		const oWord = orig[i];
		if (oWord === word) continue;
		const wordH = isCyrillic
			? word.replace(G_REGEX, ($0) => gobj[$0 as G_REGEX_MATCH])
			: word;
		if (oWord === wordH) continue;
		if (!/\(/.test(word)) {
			if (word.length === oWord.length) {
				const wordLetters = word.split('');
				for (let j = 0; j < wordLetters.length; j++) {
					if (wordH[j] !== oWord[j]) wordLetters[j] = applyF(wordLetters[j]);
				}
				text[i] = wordLetters.join('');
				continue;
			}
			if (isCyrillic) {
				const word1 = word.replace(/ь/g, '');
				switch (oWord) {
					case word1:
						text[i] = word.replace(/ь/g, applyF('ь'));
						continue;
					case word1 + 'ь':
						text[i] = word.slice(0, -1).replace(/ь/g, applyF('ь')) + 'ь';
						continue;
				}
			}
		}

		const oWordEnd = oWord.length - 1;
		let fromStart = 0;
		let fromWordEnd = word.length - 1;
		let fromOWordEnd = oWordEnd;

		while (wordH[fromStart] === oWord[fromStart]) ++fromStart;
		while (wordH[fromWordEnd] === oWord[fromOWordEnd]) {
			--fromWordEnd;
			--fromOWordEnd;
		}

		if (oWord.length < word.length) {
			if (fromOWordEnd === oWordEnd) {
				text[i] = applyF(word);
				continue;
			}
			if (fromWordEnd < 0) fromWordEnd = 0;
		}

		text[i] =
			word.slice(0, fromStart) +
			applyF(word.slice(fromStart, fromWordEnd + 1)) +
			word.slice(fromWordEnd + 1);
	}

	return text;
};

const toTarask: ToTarask = (
	text,
	replaceWithDict,
	wordlist,
	softers,
	afterTarask
) => {
	text = replaceWithDict(text, wordlist);
	softening: do {
		text = replaceWithDict(text, softers);
		for (const [pattern, result] of softers)
			if (result !== '$1дзьдз' && pattern.test(text)) continue softening;
		break;
	} while (true);

	return replaceWithDict(text, afterTarask);
};

const replaceWithDict: ReplaceWithDict = (text, dict = []) => {
	for (const [pattern, result] of dict)
		text = text.replace(
			pattern,
			//@ts-ignore
			result
		);

	return text;
};

type Vowel = 'а' | 'е' | 'ё' | 'і' | 'о' | 'у' | 'ы' | 'э' | 'ю' | 'я';

type ToJ = <TVowel extends `${Vowel} `, TU extends '' | 'ў'>(
	vow: TVowel,
	shortU: TU
) => `${TVowel}й ${TU extends 'ў' ? 'у' : ''}`;

const toJ: ToJ = (vow, shortU) =>
	(vow + 'й ' + (shortU ? 'у' : '')) as ReturnType<ToJ>;

const replaceIbyJ = (text: string, always = false) =>
	text.replace(
		/([аеёіоуыэюя] )і (ў?)/g,
		always
			? ($0, $1, $2) => toJ($1, $2)
			: ($0, $1, $2) => (Math.random() >= 0.5 ? toJ($1, $2) : $0)
	);

const finalizer = {
	html: (text: string) =>
		text
			.replace(OPTIONAL_WORDS_REGEX, ($0) => {
				const options = $0.slice(1, -1).split('|');
				const main = options.shift();
				return `<tarL data-l='${options}'>${main}</tarL>`;
			})
			.replace(/ \n /g, '<br>'),
	nonHtml(text: string, options: TaraskOptions['nonHtml']) {
		if (
			isObject(options) &&
			options.variations &&
			options.variations !== VARIATION.ALL
		) {
			const WORD_INDEX = options.variations;
			const replacer = ($0: string) => $0.slice(1, -1).split('|')[WORD_INDEX];
			text = text.replace(
				OPTIONAL_WORDS_REGEX,
				options.nodeColors
					? ($0) => tagApplications.nonHtml.L(replacer($0))
					: replacer
			);
		}
		return text.replace(/&#40/g, '(');
	},
};
