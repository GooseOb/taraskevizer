import {
	wordlist,
	softeners,
	arabLetters,
	latinLetters,
	latinLettersUpperCase,
	gobj,
	latinLettersJi,
	latinLettersUpperCaseJi,
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
const OPTIONAL_WORDS_REGEX = /\([^)]*?\)/g;
const G_REGEX = /[Ґґ]/g;
type G_REGEX_MATCH = 'Ґ' | 'ґ';

export const ALPHABET = {
	CYRILLIC: 0,
	LATIN: 1,
	ARABIC: 2,
	LATIN_JI: 3,
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
	[ALPHABET.LATIN_JI]: latinLettersJi,
};
const lettersUpperCase: AlphabetDependentDict = {
	[ALPHABET.LATIN]: latinLettersUpperCase,
	[ALPHABET.LATIN_JI]: latinLettersUpperCaseJi,
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
const finalize = (text: string, newLine: string) =>
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
					? word.replace(/[^)]*?(?=\))/, ($0) =>
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

const toJ = (shortU: '' | 'ў') => 'й ' + (shortU ? 'у' : '');

const replaceIbyJ = (text: string, always = false) =>
	text.replace(
		/(?<=[аеёіоуыэюя] )і (ў?)/g,
		always
			? ($0, $1) => toJ($1)
			: ($0, $1) => (Math.random() >= 0.5 ? toJ($1) : $0)
	);

export const __tarask__ = {
	wordlist,
	softeners,
	replaceWithDict,
	afterTarask,
} as const;

const convertAlphabet = (text: string, abc: Alphabet) =>
	replaceWithDict(replaceWithDict(text, letters[abc]), lettersUpperCase[abc]);

export class Taraskevizer {
	public abc: Alphabet = ALPHABET.CYRILLIC;
	public j: OptionJ = REPLACE_J.NEVER;
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
				'doEscapeCapitalized',
			] satisfies (keyof TaraskOptions)[])
				if (prop in general) this[prop] = general[prop] as never;
		}
		if (options.OVERRIDE_taraskevize)
			this.taraskevize = options.OVERRIDE_taraskevize;
		Object.assign(this.html, options.html);
		Object.assign(this.nonHtml, options.nonHtml);
	}

	public convert(text: string) {
		const wrapInColorOf = wrappers.ansiColors;
		const isCyrillic = this.abc === ALPHABET.CYRILLIC;
		const noFixArr: string[] = [];
		const { splitted, splittedOrig } = this.process(
			this.prepare(text, noFixArr, '<')
		);
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

		return finalize(applyNoFix(noFixArr, text).replace(/&#40/g, '('), '\n');
	}

	public convertToHtml(text: string) {
		const wrapInTag = wrappers.html;
		const isCyrillic = this.abc === ALPHABET.CYRILLIC;
		const noFixArr: string[] = [];
		const { splitted, splittedOrig } = this.process(
			this.prepare(text, noFixArr, '&lt;')
		);
		highlightChanges(splitted, splittedOrig, isCyrillic, wrapInTag.fix);
		text = join(splitted);
		if (isCyrillic)
			text = replaceG(
				text,
				this.html.g
					? wrapInTag.letterH('$&')
					: ($0) => wrapInTag.letterH(gobj[$0])
			);

		return finalize(
			applyNoFix(noFixArr, text).replace(OPTIONAL_WORDS_REGEX, ($0) => {
				const options = $0.slice(1, -1).split('|');
				const main = options.shift();
				return `<tarL data-l='${options}'>${main}</tarL>`;
			}),
			'<br>'
		);
	}

	private prepare(
		text: string,
		noFixArr: string[],
		LEFT_ANGLE_BRACKET: string,
		doEscapeCapitalized = this.doEscapeCapitalized
	) {
		text = ` ${text.trim()} `.replace(/\ue0ff/g, '');
		if (doEscapeCapitalized)
			text = text.replace(
				/(?!<=\p{Lu} )(\p{Lu}{2}[\p{Lu}\s\dʼ'\p{P}]*)(?!= \p{Lu})/gu,
				'<*.$1>'
			);
		return text
			.replace(/<(\*?)([,.]?)([^>]*?)>/gs, ($0, $1, $2, $3) => {
				if ($2 === ',') return LEFT_ANGLE_BRACKET + $3 + '>';
				if ($1)
					$3 = restoreCase(
						[replaceWithDict($3.toLowerCase(), letters[this.abc])],
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
	}

	public convertAlphabetOnly(text: string) {
		const noFixArr: string[] = [];
		return finalize(
			applyNoFix(
				noFixArr,
				convertAlphabet(this.prepare(text, noFixArr, '<', false), this.abc)
			)
				.replace(/&nbsp;/g, ' ')
				.replace(/ (\p{P}|\p{S}|\d|&#40) /gu, '$1'),
			'\n'
		);
	}

	private process(text: string): {
		splittedOrig: string[];
		splitted: string[];
	} {
		const { abc, j } = this;

		const splittedOrig = convertAlphabet(text, abc).split(' ');

		text = this.taraskevize(text.toLowerCase());
		if (j && abc !== ALPHABET.LATIN_JI)
			text = replaceIbyJ(text, j === REPLACE_J.ALWAYS);
		text = replaceWithDict(text, letters[abc]);

		let splitted = text.split(' ');
		if (abc !== ALPHABET.ARABIC) splitted = restoreCase(splitted, splittedOrig);
		return { splittedOrig, splitted };
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
