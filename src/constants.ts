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
export const REPLACE_J: { [p in Uppercase<OptionJ>]: OptionJ } = {
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
export const VARIATION: { [p in Uppercase<Variation>]: Variation } = {
	NO: 'no',
	FIRST: 'first',
	ALL: 'all',
};
