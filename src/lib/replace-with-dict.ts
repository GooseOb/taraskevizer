import type { ExtendedDict } from './types';

export const replaceWithDict = (text: string, dict: ExtendedDict = []) => {
	for (const [pattern, result] of dict) text = text.replace(pattern, result);

	return text;
};
