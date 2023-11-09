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
	ExtendedDict,
	ToTarask,
	ReplaceWithDict,
	NonHtmlOptions,
	HtmlOptions,
	TaraskOptions,
	DeepPartialReadonly,
} from './types';
import * as debug from './tools.debug';

const isUpperCase = (str: string): boolean => str === str.toUpperCase();

const getLastLetter = (word: string, i: number) => {
	for (let i = word.length - 1; i >= 0; i--)
		if (/\p{L}/u.test(word[i])) return word[i];
	throw new Error(`the last letter of the word ${word} not found. index: ${i}`);
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

let noFix: string[] = [];
const process = (
	text: string,
	LEFT_ANGLE_BRACKET: string,
	options: Readonly<TaraskOptions>
): { splittedOrig: string[]; splitted: string[] } => {
	const { abc, j, OVERRIDE_toTarask: _toTarask = toTarask } = options;
	const noFix: string[] = [];

	text = ` ${text.trim()} `
		.replace(/\ufffd/g, '')
		.replace(/<([,.]?)([.\s]*?)>/g, ($0, $1, $2) => {
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

	text = _toTarask(
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
	return { splittedOrig, splitted };
};
const applyNoFix = (text: string) => {
	if (noFix.length)
		text = text.replace(NOFIX_REGEX, () => noFix.shift() as string);
	noFix = [];
	return text;
};

const join = (textArr: string[]): string =>
	textArr
		.join(' ')
		.replace(/&nbsp;/g, ' ')
		.replace(/ (\p{P}|\p{S}|\d|&#40) /gu, '$1');
const finilize = (text: string, newLine: string) =>
	text.replace(/ \t /g, '\t').replace(/ \n /g, newLine).trim();

const replaceG = (
	text: string,
	replacer: string | ((g: keyof typeof gobj) => string)
) =>
	text.replace(
		G_REGEX,
		// @ts-ignore
		replacer
	);

const getCompletedOptions = (
	options?: DeepPartialReadonly<TaraskOptions>
): TaraskOptions => ({
	abc: 0,
	j: 0,
	...options,
});

export const taraskToHtml: Tarask<HtmlOptions> = (
	text,
	taraskOptions,
	htmlOptions = {}
) => {
	const options = getCompletedOptions(taraskOptions);
	const apply = tagApplications.html;
	const isCyrillic = options.abc === ALPHABET.CYRILLIC;
	const { splitted, splittedOrig } = process(text, '&lt;', options);
	addTags(splitted, splittedOrig, isCyrillic, apply.F);
	text = join(splitted);
	if (isCyrillic)
		text = replaceG(
			text,
			htmlOptions.g ? apply.H('$&') : ($0) => apply.H(gobj[$0])
		);

	return finilize(
		applyNoFix(text).replace(OPTIONAL_WORDS_REGEX, ($0) => {
			const options = $0.slice(1, -1).split('|');
			const main = options.shift();
			return `<tarL data-l='${options}'>${main}</tarL>`;
		}),
		'<br>'
	);
};

export const tarask: Tarask<NonHtmlOptions> = (
	text,
	taraskOptions,
	nonHtmlOptions = {}
) => {
	const options = getCompletedOptions(taraskOptions);
	const apply = tagApplications.nonHtml;
	const isCyrillic = options.abc === ALPHABET.CYRILLIC;
	const { splitted, splittedOrig } = process(text, '&lt;', options);
	if (nonHtmlOptions.nodeColors)
		addTags(splitted, splittedOrig, isCyrillic, apply.F);
	text = join(splitted);
	if (isCyrillic && (nonHtmlOptions.h || nonHtmlOptions.nodeColors))
		text = replaceG(
			text,
			nonHtmlOptions.nodeColors
				? nonHtmlOptions.h
					? ($0) => apply.H(gobj[$0])
					: apply.H('$&')
				: ($0) => gobj[$0]
		);

	if (
		'variations' in nonHtmlOptions &&
		nonHtmlOptions.variations !== VARIATION.ALL
	) {
		const wordIndex = nonHtmlOptions.variations ?? 0;
		const replacer = ($0: string) => $0.slice(1, -1).split('|')[wordIndex];
		text = text.replace(
			OPTIONAL_WORDS_REGEX,
			nonHtmlOptions.nodeColors ? ($0) => apply.L(replacer($0)) : replacer
		);
	}

	return finilize(applyNoFix(text).replace(/&#40/g, '('), '\n');
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
		} else if (isUpperCase(getLastLetter(oWord, i))) {
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

const addTags = (
	text: string[],
	orig: readonly string[],
	isCyrillic: boolean,
	applyF: (content: string) => string
): void => {
	for (let i = 0; i < text.length; i++) {
		const word = text[i];
		const oWord = orig[i];
		if (oWord === word) continue;
		const wordH = isCyrillic ? replaceG(word, ($0) => gobj[$0]) : word;
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

	// return text;
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
