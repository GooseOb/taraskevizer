import { dictFrom } from './lib';
import { iwords } from './iwords';

export const noSoften = dictFrom.nonGlobal([
	[/масфільм/, 'мас\ue0ffфільм'],
	[/пэндзлік/, 'пэндз\ue0ffлік'],
	[/ (п|н|пе?р)адзьдз/, ' $1\ue0ffаддз'],
]);

export const softeners = dictFrom.raw([
	[/([лнц])\1(?=[еёіюяь])/, '$1ь$1'],
	[/(\S\S)дз?дз(?=[еёіюяь])/, '$1дзьдз'],
	[/адзьдз(?=[ея]л)/, 'аддз'],
	[/надзьдзіман/, 'наддзіман'],
	[/з(?=(?:[бвзлмнц]|дз)[еёіюяь])/, 'зь'],
	[/с(?=[бвлмнпсфц][еёіюяь])/, 'сь'],
	[/ц(?=[вм][еёіюяь])/, 'ць'],
	[
		`( (?:б[ея]|пра|цера)?з) (?=\\(?)(?=[еёюяь]|([бвзйлмнпстфц]|дз)[еёіюяь]|імі? |іх(?:ні)?|і(?:${iwords}))`,
		'$1ь ',
	],
	[/([сз])ʼ(?=[яюеё])/, '$1ь'],
]);
