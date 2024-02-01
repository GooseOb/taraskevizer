import { ALPHABET, REPLACE_J, TaraskOptions } from '../../src';

type ItoJCase = [
	[string, TaraskOptions['j']?, TaraskOptions['abc']?],
	string
][];

export default [
	[['Яна і ён'], 'Яна і ён'],
	[['Яна і ён', REPLACE_J.NEVER], 'Яна і ён'],
	[['Яна і ён', REPLACE_J.ALWAYS], 'Яна й ён'],
	[['Яна і ён', undefined, ALPHABET.LATIN], 'Jana i jon'],
	[['Яна і ён', REPLACE_J.NEVER, ALPHABET.LATIN], 'Jana i jon'],
	[['Яна і ён', REPLACE_J.ALWAYS, ALPHABET.LATIN], 'Jana j jon'],
] satisfies ItoJCase as ItoJCase;
