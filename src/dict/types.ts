export type Dict<Pattern = RegExp> = readonly (readonly [Pattern, string])[];

export type CallableDict<Pattern = RegExp> = {
	(value: string): string;
	value: Dict<Pattern>;
};

/**
 * Useful if you want to modify the dictionary.
 *
 * Consider converting it to the type {@link Dict} or {@link CallableDict}
 * if you no longer need to modify it.
 */
export type WritableDict<Pattern = RegExp> = [Pattern, string][];
