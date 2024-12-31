export default [
	// number of letters is the same -> changed letters
	[['планета'], 'пл[я]н[э]та'],
	[['ааа', 'ббб'], '[ббб]'],
	// only difference are letters ь -> ь
	[['смех'], 'с[ь]мех'],
	[['балкон'], 'бал[ь]кон'],
	// words have different lengths -> from the first changed letter to the last one
	[['брэст'], '[(брэст|берасьце)]'],
	// but if possible, get more precise
	[['балонья'], 'бал[ёньн]я'],
	[['бернардзінцы'], 'б[эрнарды]нцы'],
	// [['іспанія'], '[гіш]панія'],
	[['мекка'], 'м[э]ка'],
	[['санкцый'], 'санкц[(ый|ыяў)]'],
	// letters in the end are removed -> last letter
	[['баторый'], 'батор[ы]'],
	[['сцэнарый'], 'сцэна[р]'],
	// letters in the beginning are removed -> first letter
	[['Віктор Гюго'], 'Віктор [Ю]ґо'],
	// letters in the middle are removed -> letters surrounding the removed ones
	[['казахскі'], 'каз[ас]кі'],
	[['płanieta', 'planeta'], 'p[lan]eta'],
] as const;
