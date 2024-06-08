export type Dict<T = RegExp> = readonly (readonly [T, string])[];

/**
 * Useful if you want to modify the dictionary.
 *
 * Consider converting it to the type {@link Dict}
 * if you no longer need to modify it.
 */
export type WritableDict<T = RegExp> = [T, string][];

/**
 * useful if you use variables in patterns.
 *
 * Don't forget to convert it to the type {@link Dict},
 * {@link lib.dictFrom} can help you with this.
 */
export type RawDict = WritableDict<string | RegExp>;
