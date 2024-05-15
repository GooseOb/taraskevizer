export const replaceG = (
	text: string,
	replacer: string | ((g: 'Ґ' | 'ґ') => string)
) =>
	text.replace(
		/[Ґґ]/g,
		// @ts-ignore
		replacer
	);
