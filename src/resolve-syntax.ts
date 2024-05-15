import { NOFIX_CHAR } from './no-fix';

export const resolveSpecialSyntax = (
	text: string,
	noFixArr: string[],
	leftAngleBracket: string,
	convertAlphavet: (text: string) => string
) => {
	let result = '';
	let endOfPart = 0;
	const counter = {
		'<': 0,
		'>': 0,
	};
	let currentPart = '';
	for (const part of text.split(/[<>]/g)) {
		endOfPart += part.length;
		++counter[text[endOfPart] as keyof typeof counter];
	}
	return text.replace(/<(\*?)([,.]?)([^>]*?)>/gs, ($0, $1, $2, $3) => {
		if ($2 === ',') return leftAngleBracket + $3 + '>';
		if ($1) $3 = convertAlphavet($3);
		noFixArr.push($2 === '.' ? $3 : leftAngleBracket + $3 + '>');
		return NOFIX_CHAR;
	});
};
