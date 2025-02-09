import type { CallableDict } from '../dict/types';

const prefix = '[debug]';

export const dict = (dict: CallableDict, regex: RegExp): CallableDict => {
	const fn: CallableDict = (text) =>
		fn.value.reduce((acc, item) => {
			if (regex.test(acc)) {
				console.log(prefix, 'replaceWithDict:', item, acc);
				process.exit(1);
			}
			return acc.replace(item[0], item[1]);
		}, text);
	fn.value = dict.value;
	return fn;
};

export const trace = <T>(...msgs: [T, ...unknown[]]) => {
	console.log(prefix, ...msgs);
	return msgs[0] as T;
};
