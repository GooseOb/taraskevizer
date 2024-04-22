import type { Dict } from '../types';
import { iwords } from './iwords';
import { RawDict } from './types';

const common = {
	lower: [
		[
			[/(?<=[аеёіоуўыэюяьʼ| >АЕЁІОУЎЫЭЮЯЬ]|^)е/, 'je'],
			[/(?<=[eаеёіоуўыэюяьʼ| >АЕЁІОУЎЫЭЮЯЬ]|^)ё/, 'jo'],
			[/(?<=[eoаеёіоуўыэюяьʼ| >АЕЁІОУЎЫЭЮЯЬ]|^)ю/, 'ju'],
			[/(?<=[eouаеёіоуўыэюяьʼ| >АЕЁІОУЎЫЭЮЯЬ]|^)я/, 'ja'],
		],
		[
			[/ʼ/, ''],
			[/ць/, 'ć'],
			[/зь/, 'ź'],
			[/сь/, 'ś'],
			[/нь/, 'ń'],
			[/ль/, 'l'],
			[/а/, 'a'],
			[/б/, 'b'],
			[/в/, 'v'],
			[/г/, 'h'],
			[/ґ/, 'g'],
			[/д/, 'd'],
			[/е/, 'ie'],
			[/ё/, 'io'],
			[/ж/, 'ž'],
			[/з/, 'z'],
			[/і/, 'i'],
			[/й/, 'j'],
			[/к/, 'k'],
			[/л/, 'ł'],
			[/м/, 'm'],
			[/н/, 'n'],
			[/о/, 'o'],
			[/п/, 'p'],
			[/р/, 'r'],
			[/с/, 's'],
			[/т/, 't'],
			[/у/, 'u'],
			[/ў/, 'ŭ'],
			[/ф/, 'f'],
			[/х/, 'ch'],
			[/ц/, 'c'],
			[/ч/, 'č'],
			[/ш/, 'š'],
			[/ы/, 'y'],
			[/э/, 'e'],
			[/ю/, 'iu'],
			[/я/, 'ia'],
			[/[łl]i([eoua])/, 'l$1'],
			[/łi/, 'li'],
		],
	],
	upper: [
		[
			[/ Е(?=[ \p{P}\d]*\p{Lu}?\p{Ll})/u, ' Je'],
			[/ Ё(?=[ \p{P}\d]*\p{Lu}?\p{Ll})/u, ' Jo'],
			[/ Ю(?=[ \p{P}\d]*\p{Lu}?\p{Ll})/u, ' Ju'],
			[/ Я(?=[ \p{P}\d]*\p{Lu}?\p{Ll})/u, ' Ja'],
		],
		[
			[/(?<=[АЕЁІОУЎЫЭЮЯЬ| ]\(?)Е/, 'JE'],
			[/(?<=[EАЕЁІОУЎЫЭЮЯЬ| ]\(?)Ё/, 'JO'],
			[/(?<=[EOАЕЁІОУЎЫЭЮЯЬ| ]\(?)Ю/, 'JU'],
			[/(?<=[EOUАЕЁІОУЎЫЭЮЯЬ| ]\(?)Я/, 'JA'],
		],
		[
			[/Е/, 'IE'],
			[/Ё/, 'IO'],
			[/Ю/, 'IU'],
			[/Я/, 'IA'],
			[/Ц[Ьь]/, 'Ć'],
			[/З[Ьь]/, 'Ź'],
			[/С[Ьь]/, 'Ś'],
			[/Н[Ьь]/, 'Ń'],
			[/Л[Ьь]/, 'L'],
			[/А/, 'A'],
			[/Б/, 'B'],
			[/В/, 'V'],
			[/Г/, 'H'],
			[/Ґ/, 'G'],
			[/Д/, 'D'],
			[/Ж/, 'Ž'],
			[/З/, 'Z'],
			[/І/, 'I'],
			[/Й/, 'J'],
			[/К/, 'K'],
			[/Л/, 'Ł'],
			[/М/, 'M'],
			[/Н/, 'N'],
			[/О/, 'O'],
			[/П/, 'P'],
			[/Р/, 'R'],
			[/С/, 'S'],
			[/Т/, 'T'],
			[/У/, 'U'],
			[/Ў/, 'Ŭ'],
			[/Ф/, 'F'],
			[/ Х(?=[\p{Ll} ])/u, ' Ch'],
			[/Х/, 'CH'],
			[/Ц/, 'C'],
			[/Ч/, 'Č'],
			[/Ш/, 'Š'],
			[/Ы/, 'Y'],
			[/Э/, 'E'],
			[/[ŁL][Ii]([AEOUaeou])/, 'L$1'],
			[/Łi/, 'Li'],
			[/ŁI/, 'LI'],
		],
	],
} satisfies Record<string, Dict[]>;

export const latinLetters: Dict = [
	...common.lower[0],
	[/ʼі/, 'ji'],
	...common.lower[1],
];

export const latinLettersUpperCase: Dict = common.upper.flat();

export const rawLatinLettersJi: RawDict = [
	[/(?<=[аеёіоуыэюяАЕЁІОУЫЭЮЯ] )і Ў/, 'j U'],
	[/(?<=[аеёіоуыэюяАЕЁІОУЫЭЮЯ] )і ў/, 'j u'],
	[/(?<=[аеёіоуыэюяАЕЁІОУЫЭЮЯ] )і /, 'j '],
	[/(?<=[аеёіоуыэюяАЕЁІОУЫЭЮЯ] )І Ў/, 'J U'],
	[/(?<=[аеёіоуыэюяАЕЁІОУЫЭЮЯ] )І ў/, 'J u'],
	[/(?<=[аеёіоуыэюяАЕЁІОУЫЭЮЯ] )І /, 'J '],
	[`і(?=${iwords})`, 'ji'],
	[`І(?=${iwords})`, 'Ji'],
	[`І(?=${iwords.toUpperCase()})`, 'JI'],
	...common.lower[0],
	[/(?<=[eouaаеёіоуўыэюяʼАЕЁІОУЎЫЭЮЯЬ] *)і/, 'ji'],
	...common.lower[1],
];

export const rawLatinLettersUpperCaseJi: RawDict = [
	...common.upper[0],
	[/(?<=[eoua] *)І(?=[ \p{P}\d]*\p{Lu}?\p{Ll})/u, 'Ji'],
	...common.upper[1],
	[/(?<=[AOEUАЕЁІОУЎЫЭЮЯ][( ]*)І/, 'JI'],
	...common.upper[2],
];
