import { ALPHABET, J, TaraskOptions } from '../../src';

type ItoJCase = [
	[string, TaraskOptions['j']?, TaraskOptions['abc']?],
	string
][];

export default [
	[['Яна і ён'], 'Яна і ён'],
	[['Яна і ён', J.NEVER], 'Яна і ён'],
	[['Яна і ён', J.ALWAYS], 'Яна й ён'],
	[['Яна і ён', undefined, ALPHABET.LATIN], 'Jana i jon'],
	[['Яна і ён', J.NEVER, ALPHABET.LATIN], 'Jana i jon'],
	[['Яна і ён', J.ALWAYS, ALPHABET.LATIN], 'Jana j jon'],
] satisfies ItoJCase as ItoJCase;
