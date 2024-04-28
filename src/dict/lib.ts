import type { Dict, RawDict, WritableDict } from './types';

export const dictFrom = {
	raw: (dict: RawDict) => {
		for (const item of dict)
			item[0] = RegExp(
				item[0],
				item[0] instanceof RegExp ? item[0].flags + 'g' : 'g'
			);
		return dict as Dict;
	},
	nonGlobal: (dict: WritableDict) => {
		for (const item of dict) item[0] = RegExp(item[0], 'g' + item[0].flags);
		return dict;
	},
} satisfies Record<string, (dictLike: WritableDict) => Dict>;

export const toOneLine = (str: string): string => str.replace(/\n/g, '|');
