export type ExtendedDict = readonly (readonly [
	RegExp,
	string | ((...substrings: string[]) => string)
])[];
