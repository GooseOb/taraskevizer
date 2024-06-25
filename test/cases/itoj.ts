import { dicts, REPLACE_J, TaraskOptions } from '../../src';

const { latin } = dicts.alphabets;

type ItoJCase = [
	[string, TaraskOptions['j']?, TaraskOptions['abc']?],
	string
][];

export default [
	[['Яна і ён'], 'Яна і ён'],
	[['Яна і ён', REPLACE_J.NEVER], 'Яна і ён'],
	[['Яна і ён', REPLACE_J.ALWAYS], 'Яна й ён'],
	[['Яна і ён', undefined, latin], 'Jana i jon'],
	[['Яна і ён', REPLACE_J.NEVER, latin], 'Jana i jon'],
	[['Яна і ён', REPLACE_J.ALWAYS, latin], 'Jana j jon'],
] satisfies ItoJCase as ItoJCase;
