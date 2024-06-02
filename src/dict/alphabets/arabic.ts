import { dictFrom } from '../lib';
import { Alphabet } from './types';

const soft = '\u0652(?=[еёіюяь])';
const presoft = '([تزكثࢮбвгджзйклмнпрстфхцчшў])\u0652?(\u0651?)';

/* not standardized */
export const arabic = {
	lower: dictFrom.raw(
		[
			[/ л[ая]/, ' لا'],
			[/л[ая]/, 'ـلا'],
			// падваеньне зычнага, шадда
			[/([бвгджзйклмнпрстфхцчшў]|д[зж])\1/, '$1\u0651'],
			// няма галоснага, сукун
			[/([бвгджзйклмнпрстфхцчшў])/, '$1\u0652'],
			// а, аліф
			[/а/, '\u0627а'],
			// першая галосная, аліф
			[/ (?=[еэыуо])/, ' \u0627'],
			['д\u0652з' + soft, 'ࢮ'],
			['з' + soft, 'ز'],
			['к' + soft, 'ك'],
			['с' + soft, 'ث'],
			['т' + soft, 'ت'],
			[/([تزكث])і/, 'ы'],
			// $2 - шадда
			[presoft + '[аяэе]', '$1$2\u064E'],
			[presoft + '[іы]', '$1$2\u0650'],
			[/ і /, ' \u0627\u0650 '],
			[presoft + '[оёую]', '$1$2\u064F'],
			[/ʼ/, 'ع'],
			[/ь/, ''],
			// [/[ьʼ]/, ''],
			[/[яе]/, 'ي\u064E'],
			[/і/, 'ي\u0650'],
			[/[ёю]/, 'ي\u064F'],
			[/[аэ]/, '\u064E'],
			[/[ыі]/, '\u0650'],
			[/[оу]/, '\u064F'],
			[/б/, 'ب'],
			[/[вў]/, 'و'],
			[/г/, 'ه'],
			[/ґ/, 'غ'],
			[/д\u0652ж/, 'ج'],
			[/д/, 'د'],
			[/ж/, 'ژ'],
			[/з/, 'ض'],
			[/й/, 'ي'],
			[/к/, 'ق'],
			[/л/, 'ل'],
			[/м/, 'م'],
			[/н/, 'ن'],
			[/п/, 'پ'],
			[/р/, 'ر'],
			[/с/, 'ص'],
			[/т/, 'ط'],
			[/ф/, 'ف'],
			[/х/, 'ح'],
			[/ц/, 'ࢯ'],
			[/ч/, 'چ'],
			[/ш/, 'ش'],
			[/,/, '،'],
			[/\?/, '؟'],
		],
		'gi'
	),
} satisfies Alphabet;
