import { replaceG } from './replace-g';
import { gobj } from '@/dict';

const replaceGByOpposite = replaceG(($0) => gobj[$0]);

export const highlightDiff = (
	text: string[],
	orig: readonly string[],
	isCyrillic: boolean,
	highlight: (content: string) => string
): void => {
	highlighting: for (let i = 0; i < text.length; i++) {
		const word = text[i];
		const oWord = orig[i];
		if (oWord === word) continue;
		const wordH = isCyrillic ? replaceGByOpposite(word) : word;
		if (oWord === wordH) continue;
		if (!word.includes('(')) {
			if (word.length === oWord.length) {
				text[i] = '';
				let j = 0;
				while (j < word.length) {
					while (wordH[j] === oWord[j]) {
						text[i] += word[j];
						++j;
						if (j === word.length) continue highlighting;
					}
					const first = j;
					while (wordH[j] !== oWord[j] && j < word.length) {
						++j;
					}
					text[i] += highlight(word.slice(first, j));
				}
				continue;
			}
			if (isCyrillic) {
				const noSoftWord = word.replace(/ь/g, '');
				switch (oWord) {
					case noSoftWord:
						text[i] = word.replace(/ь/g, highlight('ь'));
						continue;
					case noSoftWord + 'ь':
						text[i] = word.replace(/ь(?!$)/g, highlight('ь'));
						continue;
				}
			}
		}

		const oWordEnd = oWord.length - 1;

		let lastI = word.length - 1;
		let lastOI = oWordEnd;
		while (wordH[lastI] === oWord[lastOI]) {
			--lastI;
			--lastOI;
		}

		if (lastI < 0) {
			// beginning removed -> first letter
			text[i] = highlight(word[0]) + word.slice(1);
			continue;
		}

		let firstI = 0;
		while (wordH[firstI] === oWord[firstI]) ++firstI;

		if (firstI === word.length) {
			// ending removed -> last letter
			text[i] = word.slice(0, lastI) + highlight(word[lastI]);
		} else if (firstI === 0 && lastOI === oWordEnd) {
			// first and last letters changed -> whole word
			text[i] = highlight(word);
			continue;
		}
		++lastI;

		if (firstI === lastI) {
			// part removed -> surrounding letters
			--firstI;
			++lastI;
		}

		text[i] =
			word.slice(0, firstI) +
			highlight(word.slice(firstI, lastI)) +
			word.slice(lastI);
	}
};
