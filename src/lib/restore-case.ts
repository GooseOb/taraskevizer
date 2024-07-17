const isUpperCase = (str: string): boolean => str === str.toUpperCase();
const initcap = (word: string) => word[0].toUpperCase() + word.slice(1);
const initcapVar = (str: string) =>
	'(' + str.slice(1).replace(/[^|)]*[|)]/g, initcap);

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
		text[i] =
			word === 'зь'
				? isUpperCase(orig[i + 1])
					? 'ЗЬ'
					: 'Зь'
				: isUpperCase(oWord.at(-1)!)
					? word.toUpperCase()
					: word.startsWith('(')
						? initcapVar(word)
						: initcap(word);
	}

	return text;
};
