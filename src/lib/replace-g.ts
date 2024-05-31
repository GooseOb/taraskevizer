export const replaceG =
	(replacer: string | ((g: 'Ґ' | 'ґ') => string)) => (text: string) =>
		text.replace(
			/[Ґґ]/g,
			// @ts-ignore
			replacer
		);
