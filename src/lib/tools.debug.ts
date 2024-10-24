import type { ExtendedDict } from './types';

export const replaceWithDict = (
	text: string,
	dict: ExtendedDict,
	regex: RegExp
) => {
	for (const item of dict) {
		const [pattern, result] = item;
		text = text.replace(pattern, result);
		if (regex.test(text)) {
			log('replaceWithDict:', item);
			process.exit(1);
		}
	}

	return text;
};

export const log = (...msgs: unknown[]) => {
	console.log('[debug]', ...msgs);
	return msgs[0];
};
