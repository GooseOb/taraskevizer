import type { CallableDict } from '../types';

/**
 * If {@link Alphabet.upper} in not defined,
 * it is assumed that the alphabet is case-insensitive,
 * so {@link Alphabet.lower} should replace both
 * upper and lower case letters.
 */
export type Alphabet<LowerPattern = RegExp, UpperPattern = LowerPattern> = {
	lower: CallableDict<LowerPattern>;
	upper?: CallableDict<UpperPattern>;
};
