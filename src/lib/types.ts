export type ExtendedDict = readonly (readonly [
	RegExp,
	string | ((...substrings: string[]) => string),
])[];

declare global {
	interface String {
		replace(...args: ExtendedDict[number]): string;
	}
}
