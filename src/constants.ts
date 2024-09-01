import type { OptionJ, Variation } from './config';

/**
 * @deprecated since 9.1.0. Will be removed in 10.0.0
 *
 * @example
 * {
 *   // deprecated
 *   j: REPLACE_J.NEVER
 *   // use this instead
 *   j: "never"
 * }
 */
export const REPLACE_J: { [P in Uppercase<OptionJ>]: Lowercase<P> } = {
	NEVER: 'never',
	RANDOM: 'random',
	ALWAYS: 'always',
};

/**
 * @deprecated since 9.1.0. Will be removed in 10.0.0
 *
 * @example
 * {
 *   // deprecated
 *   variations: VARIATION.NO
 *   // use this instead
 *   variations: "no"
 * }
 */
export const VARIATION: { [P in Uppercase<Variation>]: Lowercase<P> } = {
	NO: 'no',
	FIRST: 'first',
	ALL: 'all',
};
