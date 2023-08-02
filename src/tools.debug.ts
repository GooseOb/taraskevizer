import { Dict } from './types';
type LogFn = (...args: any[]) => void;
declare const console: { log: LogFn };

export function replaceWithDict(
	text: string,
	dict: Dict = null,
	regex: RegExp
): string {
	for (const key in dict) {
		text = text.replace(dict[key], key);
		if (regex.test(text)) throw `'${key}': /${dict[key]}/`;
	}

	return text;
}

export const log: LogFn = (...msgs) => {
	console.log(...msgs);
};
