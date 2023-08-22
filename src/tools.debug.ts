import { Dict } from './types';

export function replaceWithDict(
	text: string,
	dict: Dict,
	regex: RegExp
): string {
	for (const item of dict) {
		const [pattern, result] = item;
		text = text.replace(pattern, result);
		if (regex.test(text)) {
			log('replaceWithDict:', item);
			process.exit(1);
		}
	}

	return text;
}

export const log = (...msgs: any[]) => {
	console.log('[debug]', ...msgs);
};
