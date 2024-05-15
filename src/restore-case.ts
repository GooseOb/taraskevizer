const isUpperCase = (str: string): boolean => str === str.toUpperCase();

const getLastLetter = (word: string, i: number) => {
	for (let i = word.length - 1; i >= 0; i--)
		if (/\p{L}/u.test(word[i])) return word[i];
	throw new Error(`the last letter of the word ${word} not found. index: ${i}`);
};

export const restoreCase = (text: string[], orig: string[]): string[] => {
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
