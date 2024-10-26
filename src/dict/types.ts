export type Callable<Value> = {
	(value: string): string;
	value: Value;
};

export type Dict<T = RegExp> = readonly (readonly [T, string])[];

export type CallableDict<T = RegExp> = Callable<Dict<T>>;

/**
 * Useful if you want to modify the dictionary.
 *
 * Consider converting it to the type {@link CallableDict}
 * if you no longer need to modify it.
 */
export type WritableDict<T = RegExp> = [T, string][];
