const isUpperCase = (str: string): boolean => str === str.toUpperCase();

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
					? 'ЗЬ' // upper
					: 'Зь' // initcap
				: isUpperCase(oWord.at(-1)!)
					? word.toUpperCase() // upper
					: word.startsWith('(') // initcap
						? word.replace(/^[^)]+/, ($0) =>
								$0.replace(/[(|]./g, ($0) => $0.toUpperCase())
							)
						: word[0].toUpperCase() + word.slice(1);
	}

	return text;
};
