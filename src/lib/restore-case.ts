const isUpperCase = (str: string): boolean => str === str.toUpperCase();

const getLastLetter = (word: string, i: number) => {
	const result = /\p{L}(?=[^\p{L}]*$)/u.exec(word);
	if (result) return result[0];
	throw new Error(
		`the last letter of the word "${word}" not found. index: ${i}`
	);
};

export const restoreCase = (text: string[], orig: readonly string[]) => {
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
			text[i] = word.startsWith('(')
				? word.replace(/[^)]*?(?=\))/, ($0) =>
						$0.replace(/[(|]./g, ($0) => $0.toUpperCase())
					)
				: word[0].toUpperCase() + word.slice(1);
		}
	}

	return text;
};
