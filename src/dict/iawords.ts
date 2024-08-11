import { iwords } from './iwords';
import { dictFrom, toOneLine } from './lib';

const ia = (word: string, words: string): string =>
	` ${word}(?= \\(?(?:[бвгджзйклмнпрстфцчшўьʼ]*[оё]|${toOneLine(words).replace(
		/\(/g,
		'(?:'
	)}|i(?:${iwords})))`;

export const iaWords = dictFrom.raw([
	/* не > ня */
	[
		ia(
			'не',
			`(вы)?(бач[ауы]
бег[ла]
блыта[юц]
брал[аі]
буд(зе|у(ць|чы)?) 
ваш
веда(л[аі]|ць|ю(ць|чы)?|ў)
дума(л[аі]|ць|ю(ць|чы)?|ў)
дур([ауы]|ай|ань|ня[ўм]?| )
зна(л[аі]|ць|ю(ць|чы)?)
кла(д |л|ў)
ма(е|ючы|ю(ць|чы)?)
ме(л|ў|ць)
мы(ц|л[аі])
нейк
пі(са|шу)(цц|чы|ць )?
ў?пэўн
ска(ж(а|уць)|за )
спал[аі]
спра[вў]
трэба 
чу[елўцю])
веды
веліч
вы(раш|гар|пад|[дп]ал[еіяю])
гор(ш|ай)
гу(к|чн|ст)
дрэнн
зьбег
іхн
карт\\S{0,4}[ \\)]
каш(ай?|амі|у|ы) 
лёгк
літар
лішн
(мен|бол|леп)(ш|ей)
мап\\S{0,4}[ \\)]
медз
мякк
наш([аы]я|ую|ых)
прыйдзеш
руш[аы]
ста(ў|не|нуць|л[аі])
таго
тое
тыя
ўвод
ўс[еёя]
фарб(а(ў|мі)?|у|ы)? 
ян `
		),
		' ня',
	],
	/* без > бяз */
	[
		ia(
			'без',
			`(вы)?(клад[ау]
мела 
мытых
піса[нц]
ска(жа|жуць|за) 
спал[аі]
чу[елўцю])
ваш
ведаў
велічы
выраш
вы[дп]але
выпад
гор(ш|ай)
гу(к|чн|ст)
дрэнн
іхн
карт\\S{0,4}[ \\)]
каш(ы|аў) 
лёгк
літар
лішн
медз
(мен|бол|леп|ш|ей)
мап\\S{0,4}[ \\)]
мякк
нейк
наш(ую|ых)
пішучы
руша
стаў
таго
ўвод
ўс[еёя]
фарб(аў|ы)? 
цукр[ау] 
ян`
		),
		' бяз',
	],
]);
