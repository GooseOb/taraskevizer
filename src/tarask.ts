import {
	wordlist,
	softers,
	arabLetters,
	latinLetters,
	latinLettersUpperCase,
	gobj,
} from './dict';
import {
	Tarask,
	TaraskAsync,
	Dict,
	AlphabetDependentDict,
	TaraskOptions,
} from './types';
import * as debug from './tools.debug';

const isUpperCase = (str: string): boolean => str === str.toUpperCase();

const NOFIX_CHAR = ' \uffff ';
const NOFIX_REGEX = new RegExp(NOFIX_CHAR, 'g');
const OPTIONAL_WORDS_REGEX = /\(.*?\)/g;
const G_REGEX = /[Ґґ]/g;

const CYRILLIC = 0,
	LATIN = 1,
	ARABIC = 2;
const J_NEVER = 0,
	J_RANDOM = 1,
	J_ALWAYS = 2;

const letters = {
	[LATIN]: latinLetters,
	[ARABIC]: arabLetters,
} satisfies AlphabetDependentDict;
const lettersUpperCase = {
	[LATIN]: latinLettersUpperCase,
} satisfies AlphabetDependentDict;
const additionalReplacements = {
	[CYRILLIC]: [
		['$1У', /([АЕЁІОУЫЭЮЯ])<tarF>Ў<\/tarF>/g],
		[' У', / <tarF>Ў<\/tarF>(?=\p{Lu})/gu],
	],
	[LATIN]: [
		['$1U', /([AEIOUY])<tarF>Ŭ<\/tarF>/g],
		[' U', / <tarF>Ŭ<\/tarF>(?=\p{Lu})/gu],
	],
	[ARABIC]: [],
} satisfies AlphabetDependentDict;

type SpecificApplyObj = Record<'F' | 'H' | 'L', (content: string) => string>;

const applyObj = {
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

export const taraskSync: Tarask = (text, options) => {
	const { abc = 0, j = 0, html = false, nonHtml = false } = options || {};
	// if (typeof html === 'object') {
	// }
	if (typeof nonHtml === 'object') {
		nonHtml.variations ||= 0;
	}
	const apply = html ? applyObj.html : applyObj.nonHtml;
	const noFix: string[] = [];

	const LEFT_ANGLE_BRACKET = html ? '&lt;' : '<';

	text = ` ${text.trim()}  `
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

	text = toTarask(text.toLowerCase());
	if (j) text = replaceIbyJ(text, j === J_ALWAYS);
	text = replaceWithDict(text, letters[abc]);

	splitted = text.split(' ');
	if (abc !== ARABIC) splitted = restoreCase(splitted, splittedOrig);
	const nodeColors = nonHtml && nonHtml.nodeColors;
	if (html || nodeColors)
		splitted = toHtmlTags(splitted, splittedOrig, abc, apply.F);

	text = splitted
		.join(' ')
		.replace(/&nbsp;/g, ' ')
		.replace(/ (\p{P}|\p{S}|\d|&#40) /gu, '$1');

	let gReplacer: undefined | string | ((...substrings: string[]) => string);
	if (html) {
		text = replaceWithDict(text, additionalReplacements[abc]);
		if (abc === CYRILLIC) {
			gReplacer = html.g ? apply.H('$&') : ($0) => apply.H(gobj[$0]);
		}
	} else if (nonHtml && abc === CYRILLIC) {
		if (nonHtml.nodeColors) {
			gReplacer = nonHtml.h ? ($0) => apply.H(gobj[$0]) : apply.H('$&');
		} else if (nonHtml.h) {
			gReplacer = ($0) => gobj[$0];
		}
	}

	// @ts-ignore
	if (gReplacer) text = text.replace(G_REGEX, gReplacer);

	if (noFix.length) text = text.replace(NOFIX_REGEX, () => noFix.shift());

	return (
		html
			? finalizer.html(text)
			: finalizer.nonHtml(text, nonHtml, apply as SpecificApplyObj)
	).trim();
};

export const tarask: TaraskAsync = (...args) =>
	new Promise((res) => res(taraskSync(...args)));

function restoreCase(text: string[], orig: string[]): string[] {
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
		} else if (isUpperCase(oWord[oWord.length - 1])) {
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
}

function toHtmlTags(
	text: string[],
	orig: string[],
	abc: TaraskOptions['abc'],
	applyF: (content: string) => string
): string[] {
	for (let i = 0; i < text.length; i++) {
		const word = text[i];
		const oWord = orig[i];
		if (oWord === word) continue;
		const wordH = word.replace(G_REGEX, ($0) => gobj[$0]);
		if (oWord === wordH) continue;
		if (!/\(/.test(word)) {
			if (word.length === oWord.length) {
				const LettersText = word.split('');
				for (let x = 0; x < LettersText.length; x++) {
					if (LettersText[x] !== oWord[x])
						LettersText[x] = applyF(LettersText[x]);
				}
				text[i] = LettersText.join('');
				continue;
			}
			if (abc === CYRILLIC) {
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
}

function toTarask(text: string): string {
	text = replaceWithDict(text, wordlist);
	loop: do {
		text = replaceWithDict(text, softers);
		for (const [result, pattern] of softers)
			if (result !== '$1дзьдз' && pattern.test(text)) continue loop;
		break;
	} while (true);

	const iaReplacer = <TStart extends ' б' | ' н', T extends string>(
		$0: `${TStart}е${T}`,
		$1: TStart,
		$2: T
	) => ($2.match(/[аеёіоуыэюя]/g)?.length === 1 ? $1 + 'я' + $2 : $0);

	return text
		.replace(/ [уў]асьнігл /g, ' уаснігл ')
		.replace(/ сь(?=нід |мі )/g, ' с')
		.replace(/( б)е(зь? \S+)/g, iaReplacer)
		.replace(/( н)е( \S+)/g, iaReplacer)
		.replace(/( (?:б[ея]|пра|цера)?з) і(\S*)/g, ($0, $1, $2) =>
			/([ая]ў|ну)$/.test($2) ? $1 + 'ь і' + $2 : $0
		);
}

function replaceWithDict(text: string, dict: Dict = []): string {
	for (const [result, pattern] of dict) text = text.replace(pattern, result);

	return text;
}

type Vow = 'а' | 'е' | 'ё' | 'і' | 'о' | 'у' | 'ы' | 'э' | 'ю' | 'я';

type ToJ = <TVow extends `${Vow} `, TU extends '' | 'ў'>(
	vow: TVow,
	shortU: TU
) => `${TVow}й ${TU extends 'ў' ? 'у' : ''}`;

const toJ: ToJ = (vow, shortU) =>
	(vow + 'й ' + (shortU ? 'у' : '')) as ReturnType<ToJ>;

function replaceIbyJ(text: string, always = false): string {
	return text.replace(
		/([аеёіоуыэюя] )і (ў?)/g,
		always
			? ($0, $1, $2) => toJ($1, $2)
			: ($0, $1, $2) => (Math.random() >= 0.5 ? toJ($1, $2) : $0)
	);
}

const finalizer = {
	html: (text: string) =>
		text
			.replace(OPTIONAL_WORDS_REGEX, ($0) => {
				const options = $0.slice(1, -1).split('|');
				const main = options.shift();
				return `<tarL data-l='${options}'>${main}</tarL>`;
			})
			.replace(/ \n /g, '<br>'),
	nonHtml(
		text: string,
		options: TaraskOptions['nonHtml'],
		apply: SpecificApplyObj
	) {
		if (options && options.variations !== 2) {
			const WORD_INDEX = options.variations;
			const replacer = ($0: string) => $0.slice(1, -1).split('|')[WORD_INDEX];
			text = text.replace(
				OPTIONAL_WORDS_REGEX,
				options.nodeColors ? ($0) => apply.L(replacer($0)) : replacer
			);
		}
		return text.replace(/&#40/g, '(');
	},
};
