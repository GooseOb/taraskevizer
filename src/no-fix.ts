export const NOFIX_CHAR = ' \ue0fe ';
const NOFIX_REGEX = new RegExp(NOFIX_CHAR, 'g');

export const applyNoFix = (arr: string[], text: string) =>
	arr.length ? text.replace(NOFIX_REGEX, () => arr.shift()!) : text;
