import { alphabets } from '../dict';
import { REPLACE_J } from '../config';
import type { TaraskStep } from './types';

const toJ = (shortU: '' | 'ў') => 'й ' + (shortU ? 'у' : '');

export const replaceIbyJ: TaraskStep = (
	text: string,
	{
		cfg: {
			general: { j, abc },
		},
	}
) =>
	j && abc !== alphabets.latinJi
		? text.replace(
				/(?<=[аеёіоуыэюя] )і (ў?)/g,
				j === REPLACE_J.ALWAYS
					? ($0, $1) => toJ($1)
					: ($0, $1) => (Math.random() >= 0.5 ? toJ($1) : $0)
			)
		: text;
