import { callableDict } from './lib';
import type { Dict } from './types';

export const commonPhonetic: Dict = [
	[/шся /g, 'сься '],
	[/здж/g, 'ждж'],
	[/ ([бд]|кнд|нот)р /g, ' $1\ue0ffр '],
	[/(\S\S[дт])р /g, '$1ар '],
];

export const phonetic = callableDict(
	(
		[
			[/сш/g, 'шш'],
			[/[зс]ч/g, 'шч'],
			[/чц(?![ьіеюя])/g, 'цц'],
			[/[жш]ц(?=[ауы] |а[хўйм] |амі )/g, 'сц'],
		] as Dict
	).concat(commonPhonetic)
);
