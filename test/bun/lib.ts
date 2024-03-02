import { describe, expect, test } from 'bun:test';

export const getLabel = (inputType: string) =>
	`\x1b[35m%${inputType}\x1b[0;2m -> \x1b[0;1m%s`;

export const testOnCases = <TInput, TOutput extends string>(
	name: string,
	fn: (arg: TInput) => TOutput,
	cases: readonly (readonly [TInput, TOutput])[],
	label = getLabel('s')
) => {
	describe(name, () => {
		test.each(cases)(label, (input, expected) => {
			expect(fn(input)).toBe(expected);
		});
	});
};

export const testOnCasesAsync = <TInput, TOutput extends string>(
	name: string,
	fn: (arg: TInput) => Promise<TOutput>,
	cases: readonly (readonly [TInput, TOutput])[],
	label = getLabel('s')
) => {
	describe(name, () => {
		test.each(cases)(label, (input, expected) => {
			fn(input).then((output) => {
				expect(output).toBe(expected);
			});
		});
	});
};
