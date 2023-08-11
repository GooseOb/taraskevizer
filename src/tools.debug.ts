import { Dict } from './types';

export function replaceWithDict(
	text: string,
	dict: Dict,
	regex: RegExp
): string {
	for (const item of dict) {
		const [result, pattern] = item;
		text = text.replace(pattern, result);
		if (regex.test(text)) {
			console.error('[replaceWithDict]', item);
			process.exit(1);
		}
	}

	return text;
}

export const log = (...msgs: any[]) => {
	console.log(...msgs);
};
