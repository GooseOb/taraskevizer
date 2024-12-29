export default [
	// if the number of letters is the same, highlight the changed letters
	['планета', 'пл[я]н[э]та'],
	// if the only difference are letters ь, highlight them
	['смех', 'с[ь]мех'],
	['балкон', 'бал[ь]кон'],
	// if the words have different lengths, highlight from the first changed letter to the last one
	['балонья', 'бал[ёньн]я'],
	['брэст', '[(брэст|берасьце)]'],
	['бернардзінцы', 'б[эрнарды]нцы'],
	['іспанія', '[гіш]панія'],
	['мекка', 'м[э]ка'],
	// if letters in the end are removed, highlight the last letter
	['баторый', 'батор[ы]'],
	['сцэнарый', 'сцэна[р]'],
	// if letters in the beginning are removed, highlight the first letter
	['Віктор Гюго', 'Віктор [Ю]ґо'],
	// if letters in the middle are removed, highlight the letters surrounding the removed ones
	['казахскі', 'каз[ас]кі'],
] as const;
