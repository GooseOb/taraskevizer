import { alphabets } from '../dict';
import type { TaraskStep } from './types';

const toJ = (shortU: '' | 'ў') => 'й ' + (shortU ? 'у' : '');

export const replaceIbyJ: TaraskStep = (ctx) => {
	const { abc, j } = ctx.cfg;
	if (j !== 'never' && abc !== alphabets.latinJi)
		ctx.text = ctx.text.replace(
			/(?<=[аеёіоуыэюя] )і (ў?)/g,
			j === 'always'
				? (_$0, $1: '' | 'ў') => toJ($1)
				: ($0, $1: '' | 'ў') => (Math.random() >= 0.5 ? toJ($1) : $0)
		);
};
