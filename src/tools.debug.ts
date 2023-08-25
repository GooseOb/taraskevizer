import { ReplaceWithDict } from './types';

type AddParameter<TFn, TNewArg> = TFn extends (
	...args: infer TArgs
) => infer TReturn
	? (...args: [...TArgs, TNewArg]) => TReturn
	: never;

export const replaceWithDict: AddParameter<ReplaceWithDict, RegExp> = (
	text,
	dict,
	regex
) => {
	for (const item of dict) {
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
};
