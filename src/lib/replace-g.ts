export const replaceG =
	(replacer: string | ((g: 'Ґ' | 'ґ') => string)) => (text: string) =>
		text.replace(
			/[Ґґ]/g,
			// @ts-expect-error TS can't infer the type of replacer
			replacer
		);
