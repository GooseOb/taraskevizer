const NOFIX_CHAR = ' \ue0fe ';
const NOFIX_REGEX = new RegExp(NOFIX_CHAR, 'g');

export const applyNoFix = (arr: string[], text: string) =>
	arr.length ? text.replace(NOFIX_REGEX, () => arr.shift()!) : text;

export const applyVariableParts = (
	text: string,
	callback: (arr: string[]) => string
) => text.replace(/\([^)]*?\)/g, ($0) => callback($0.slice(1, -1).split('|')));

export const resolveSpecialSyntax = (
	text: string,
	noFixArr: string[],
	leftAngleBracket: string,
	convertAlphavet: (text: string) => string,
	doEscapeCaps: boolean
) => {
	const escapeCapsIfNeeded = (text: string) =>
		doEscapeCaps
			? text.replace(/(?!<=\p{Lu} )\p{Lu}{2}[\p{Lu} ]*(?!= \p{Lu})/gu, ($0) => {
					noFixArr.push(convertAlphavet($0));
					return NOFIX_CHAR;
				})
			: text;
	const parts = text.split(/(?=[<>])/g);
	if (parts.length === 1) return escapeCapsIfNeeded(text);
	let result = text.startsWith('<') ? '' : escapeCapsIfNeeded(parts.shift()!);
	let depth = 0;
	let currentPart = '';
	for (const part of parts) {
		if (part.startsWith('<')) {
			++depth;
			currentPart += part;
		} else if (depth) {
			--depth;
			if (depth) {
				currentPart += part;
			} else {
				let char = '';
				const abc = currentPart[1] === '*';
				if (abc) {
					char = currentPart[2];
					currentPart = convertAlphavet(
						currentPart.slice(char === ',' || char === '.' ? 3 : 2)
					);
				} else {
					char = currentPart[1];
					currentPart = currentPart.slice(2);
				}
				let toAddToResult = NOFIX_CHAR;
				switch (char) {
					case '.':
						noFixArr.push(currentPart);
						break;
					case ',':
						toAddToResult = leftAngleBracket + currentPart + '>';
						break;
					default:
						noFixArr.push('<' + (abc ? '' : char) + currentPart + '>');
				}
				result += toAddToResult + escapeCapsIfNeeded(part.slice(1));
				currentPart = '';
			}
		} else {
			result += escapeCapsIfNeeded(part);
		}
	}
	return result + escapeCapsIfNeeded(currentPart);
};
