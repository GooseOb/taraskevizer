import type { ExtendedDict } from './types';

type AddParameter<TFn, TNewArg> = TFn extends (
	...args: infer TArgs
) => infer TReturn
	? (...args: [...TArgs, TNewArg]) => TReturn
	: never;

export const replaceWithDict = (
	text: string,
	dict: ExtendedDict,
	regex: RegExp
) => {
	for (const item of dict!) {
		const [pattern, result] = item;
		text = text.replace(
			pattern,
			//@ts-ignore
			result
		);
		if (regex.test(text)) {
			log('replaceWithDict:', item);
			process.exit(1);
		}
	}

	return text;
};

export const log = (...msgs: any[]) => {
	console.log('[debug]', ...msgs);
	return msgs[0];
};
