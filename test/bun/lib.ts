import { describe, expect, test } from 'bun:test';

export const getLabel = (inputType: string) =>
	`\x1b[35m%${inputType}\x1b[0;2m -> \x1b[0;1;35m%s`;

export const testOnCases = <TInput, TOutput extends string>(
	name: string,
	fn: (arg: TInput) => TOutput,
	cases: readonly (readonly [TInput, TOutput])[],
	label = getLabel('s')
) => {
	describe(name, () => {
		test.each(cases)(label, (arg, expected) => {
			expect(fn(arg)).toBe(expected);
		});
	});
};
