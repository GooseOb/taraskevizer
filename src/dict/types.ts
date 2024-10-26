export type Dict<T = RegExp> = readonly (readonly [T, string])[];

export type CallableDict<T = RegExp> = {
	(value: string): string;
	value: Dict<T>;
};

/**
 * Useful if you want to modify the dictionary.
 *
 * Consider converting it to the type {@link Dict} or {@link CallableDict}
 * if you no longer need to modify it.
 */
export type WritableDict<T = RegExp> = [T, string][];
