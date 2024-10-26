import type { CallableDict } from '../dict/types';

const prefix = '[debug]';

export const dict = (dict: CallableDict, regex: RegExp): CallableDict => {
	const fn: CallableDict = (text) =>
		fn.value.reduce((acc, item) => {
			if (regex.test(text)) {
				console.log(prefix, 'replaceWithDict:', item);
				process.exit(1);
			}
			return acc.replace(item[0], item[1]);
		}, text);
	fn.value = dict.value;
	return fn;
};

export const log = (...msgs: unknown[]) => {
	console.log(prefix, ...msgs);
	return msgs[0];
};
