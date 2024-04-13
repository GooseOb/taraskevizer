import {
	wordlist,
	softeners,
	arabLetters,
	latinLetters,
	latinLettersUpperCase,
	gobj,
} from './dict';
import type {
	AlphabetDependentDict,
	ExtendedDict,
	NonHtmlOptions,
	HtmlOptions,
	TaraskOptions,
	DeepPartialReadonly,
	Alphabet,
	OptionJ,
	Variation,
} from './types';
import * as debug from './tools.debug';

const isUpperCase = (str: string): boolean => str === str.toUpperCase();

const getLastLetter = (word: string, i: number) => {
	for (let i = word.length - 1; i >= 0; i--)
		if (/\p{L}/u.test(word[i])) return word[i];
	throw new Error(`the last letter of the word ${word} not found. index: ${i}`);
};

const NOFIX_CHAR = ' \ue0fe ';
const NOFIX_REGEX = new RegExp(NOFIX_CHAR, 'g');
const OPTIONAL_WORDS_REGEX = /\(.*?\)/g;
const G_REGEX = /[Ґґ]/g;
type G_REGEX_MATCH = 'Ґ' | 'ґ';

export const ALPHABET = {
	CYRILLIC: 0,
	LATIN: 1,
	ARABIC: 2,
} as const satisfies Record<string, Alphabet>;
export const REPLACE_J = {
	NEVER: 0,
	RANDOM: 1,
	ALWAYS: 2,
} as const satisfies Record<string, OptionJ>;
export const VARIATION = {
	NO: 0,
	FIRST: 1,
	ALL: 2,
} as const satisfies Record<string, Variation>;

const letters: AlphabetDependentDict = {
	[ALPHABET.LATIN]: latinLetters,
	[ALPHABET.ARABIC]: arabLetters,
};
const lettersUpperCase: AlphabetDependentDict = {
	[ALPHABET.LATIN]: latinLettersUpperCase,
};

const wrappers = {
	html: {
		fix: (content) => `<tarF>${content}</tarF>`,
		letterH: (content) => `<tarH>${content}</tarH>`,
	},
	ansiColors: {
		fix: (content) => `\x1b[32m${content}\x1b[0m`,
		variable: (content) => `\x1b[35m${content}\x1b[0m`,
	},
} satisfies Record<
	'html' | 'ansiColors',
	{ [p in 'fix' | 'letterH' | 'variable']?: (content: string) => string }
>;

const iaReplacer = ($0: string, $1: string, $2: string) =>
	$2.match(/[аеёіоуыэюя]/g)?.length === 1 ? $1 + 'я' + $2 : $0;

const afterTarask: ExtendedDict = [
	[/( б)е(зь? \S+)/g, iaReplacer],
	[/( н)е( \S+)/g, iaReplacer],
	[
		/( (?:б[ея]|пра|цера)?з) і(\S*)/g,
		($0, $1, $2) => (/([ая]ў|ну)$/.test($2) ? $1 + 'ь і' + $2 : $0),
	],
];

const applyNoFix = (arr: string[], text: string) =>
	arr.length ? text.replace(NOFIX_REGEX, () => arr.shift()!) : text;

const join = (textArr: string[]): string =>
	textArr
		.join(' ')
		.replace(/&nbsp;/g, ' ')
		.replace(/ (\p{P}|\p{S}|\d|&#40) /gu, '$1');
const finilize = (text: string, newLine: string) =>
	text.replace(/ \t /g, '\t').replace(/ \n /g, newLine).trim();

const replaceG = (
	text: string,
	replacer: string | ((g: G_REGEX_MATCH) => string)
) =>
	text.replace(
		G_REGEX,
		// @ts-ignore
		replacer
	);

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

const highlightChanges = (
	text: string[],
	orig: readonly string[],
	isCyrillic: boolean,
	highlight: (content: string) => string
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
					if (wordH[j] !== oWord[j]) wordLetters[j] = highlight(wordLetters[j]);
				}
				text[i] = wordLetters.join('');
				continue;
			}
			if (isCyrillic) {
				const word1 = word.replace(/ь/g, '');
				switch (oWord) {
					case word1:
						text[i] = word.replace(/ь/g, highlight('ь'));
						continue;
					case word1 + 'ь':
						text[i] = word.slice(0, -1).replace(/ь/g, highlight('ь')) + 'ь';
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
				text[i] = highlight(word);
				continue;
			}
			if (fromWordEnd < 0) fromWordEnd = 0;
		}

		text[i] =
			word.slice(0, fromStart) +
			highlight(word.slice(fromStart, fromWordEnd + 1)) +
			word.slice(fromWordEnd + 1);
	}
};

const replaceWithDict = (text: string, dict: ExtendedDict = []) => {
	for (const [pattern, result] of dict)
		text = text.replace(
			pattern,
			//@ts-ignore
			result
		);

	return text;
};

const toJ = (vow: `${string} `, shortU: '' | 'ў') =>
	vow + 'й ' + (shortU ? 'у' : '');

const replaceIbyJ = (text: string, always = false) =>
	text.replace(
		/([аеёіоуыэюя] )і (ў?)/g,
		always
			? ($0, $1, $2) => toJ($1, $2)
			: ($0, $1, $2) => (Math.random() >= 0.5 ? toJ($1, $2) : $0)
	);

export const __tarask__ = {
	wordlist,
	softeners,
	replaceWithDict,
	afterTarask,
} as const;

export const convertAlphabet = (
	text: string,
	abc: Alphabet,
	jiInLatin: boolean
) =>
	replaceWithDict(replaceWithDict(text, letters[abc]), lettersUpperCase[abc]);

const moveFromTo = <TKey extends string, TValue>(
	prop: TKey,
	from: { readonly [p in TKey]?: TValue },
	to: { [p in TKey]: TValue }
) => {
	if (prop in from) to[prop] = from[prop]!;
};

export class Taraskevizer {
	public abc: Alphabet = ALPHABET.CYRILLIC;
	public j: OptionJ = REPLACE_J.NEVER;
	public ji = true;
	public doEscapeCapitalized = true;
	public html = {
		g: false,
	};
	public nonHtml = {
		h: false,
		ansiColors: false,
		variations: VARIATION.ALL as Variation,
	};

	constructor(
		options?: DeepPartialReadonly<{
			general: TaraskOptions;
			html: HtmlOptions;
			nonHtml: NonHtmlOptions;
			OVERRIDE_taraskevize(this: Taraskevizer, text: string): string;
		}>
	) {
		if (!options) return;
		const general = options.general;
		if (general) {
			for (const prop of [
				'abc',
				'j',
				'ji',
				'doEscapeCapitalized',
			] satisfies (keyof TaraskOptions)[])
				moveFromTo(prop, general, this);
		}
		if (options.OVERRIDE_taraskevize)
			this.taraskevize = options.OVERRIDE_taraskevize;
		Object.assign(this.html, options.html);
		Object.assign(this.nonHtml, options.nonHtml);
	}

	public convert(text: string) {
		const wrapInColorOf = wrappers.ansiColors;
		const isCyrillic = this.abc === ALPHABET.CYRILLIC;
		const { splitted, splittedOrig, noFixArr } = this.process(text, '<');
		if (this.nonHtml.ansiColors)
			highlightChanges(splitted, splittedOrig, isCyrillic, wrapInColorOf.fix);
		text = join(splitted);
		if (isCyrillic && (this.nonHtml.h || this.nonHtml.ansiColors))
			text = replaceG(
				text,
				this.nonHtml.ansiColors
					? this.nonHtml.h
						? ($0) => wrapInColorOf.variable(gobj[$0])
						: wrapInColorOf.variable('$&')
					: ($0) => gobj[$0]
			);

		if (
			'variations' in this.nonHtml &&
			this.nonHtml.variations !== VARIATION.ALL
		) {
			const wordIndex = this.nonHtml.variations ?? 0;
			const replacer = ($0: string) => $0.slice(1, -1).split('|')[wordIndex];
			text = text.replace(
				OPTIONAL_WORDS_REGEX,
				this.nonHtml.ansiColors
					? ($0) => wrapInColorOf.variable(replacer($0))
					: replacer
			);
		}

		return finilize(applyNoFix(noFixArr, text).replace(/&#40/g, '('), '\n');
	}

	public convertToHtml(text: string) {
		const wrapInTag = wrappers.html;
		const isCyrillic = this.abc === ALPHABET.CYRILLIC;
		const { splitted, splittedOrig, noFixArr } = this.process(text, '&lt;');
		highlightChanges(splitted, splittedOrig, isCyrillic, wrapInTag.fix);
		text = join(splitted);
		if (isCyrillic)
			text = replaceG(
				text,
				this.html.g
					? wrapInTag.letterH('$&')
					: ($0) => wrapInTag.letterH(gobj[$0])
			);

		return finilize(
			applyNoFix(noFixArr, text).replace(OPTIONAL_WORDS_REGEX, ($0) => {
				const options = $0.slice(1, -1).split('|');
				const main = options.shift();
				return `<tarL data-l='${options}'>${main}</tarL>`;
			}),
			'<br>'
		);
	}

	private process(
		text: string,
		LEFT_ANGLE_BRACKET: string
	): { splittedOrig: string[]; splitted: string[]; noFixArr: string[] } {
		const { abc, j, ji } = this;
		const noFixArr: string[] = [];
		text = ` ${text.trim()} `.replace(/\ue0ff/g, '');
		if (this.doEscapeCapitalized)
			text = text.replace(/(?!<=\p{Lu} )(\p{Lu}{2,})(?!= \p{Lu})/gu, '<*.$1>');
		text = text
			.replace(/<(\*?)([,.]?)(.*?)>/gs, ($0, $1, $2, $3) => {
				if ($2 === ',') return LEFT_ANGLE_BRACKET + $3 + '>';
				if ($1)
					$3 = restoreCase(
						[replaceWithDict($3.toLowerCase(), letters[abc])],
						[$3]
					);
				noFixArr.push($2 === '.' ? $3 : LEFT_ANGLE_BRACKET + $3 + '>');
				return NOFIX_CHAR;
			})
			.replace(/г'(?![еёіюя])/g, 'ґ')
			.replace(/([\n\t])/g, ' $1 ')
			.replace(/ - /g, ' — ')
			.replace(/(\p{P}|\p{S}|\d)/gu, ' $1 ')
			.replace(/ ['`’] (?=\S)/g, 'ʼ')
			.replace(/\(/g, '&#40');

		let splittedOrig: string[], splitted: string[];
		splittedOrig = convertAlphabet(text, abc, ji).split(' ');

		text = this.taraskevize(text.toLowerCase());
		if (j) text = replaceIbyJ(text, j === REPLACE_J.ALWAYS);
		text = replaceWithDict(text, letters[abc]);

		splitted = text.split(' ');
		if (abc !== ALPHABET.ARABIC) splitted = restoreCase(splitted, splittedOrig);
		return { splittedOrig, splitted, noFixArr };
	}

	protected taraskevize(text: string) {
		text = replaceWithDict(text, wordlist);
		softening: do {
			text = replaceWithDict(text, softeners);
			for (const [pattern, result] of softeners)
				if (result !== '$1дзьдз' && pattern.test(text)) continue softening;
			break;
		} while (true);

		return replaceWithDict(text, afterTarask);
	}
}
