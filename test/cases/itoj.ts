import { dicts, TaraskConfig } from '../../src';

const { latin } = dicts.alphabets;

type ItoJCase = [[string, TaraskConfig['j']?, TaraskConfig['abc']?], string][];

export default [
	[['Яна і ён'], 'Яна і ён'],
	[['Яна і ён', 'never'], 'Яна і ён'],
	[['Яна і ён', 'always'], 'Яна й ён'],
	[['Яна і ён', undefined, latin], 'Jana i jon'],
	[['Яна і ён', 'never', latin], 'Jana i jon'],
	[['Яна і ён', 'always', latin], 'Jana j jon'],
] satisfies ItoJCase as ItoJCase;
