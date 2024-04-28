export type Dict<T = RegExp> = readonly (readonly [T, string])[];
export type WritableDict<T = RegExp> = [T, string][];
export type RawDict = WritableDict<string | RegExp>;
