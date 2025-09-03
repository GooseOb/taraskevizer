/**
 * This module exports all the steps
 * that are used in builtin pipelines.
 *
 * A step implements the {@link TaraskStep} type.
 *
 * Some steps are configurable, they take
 * additional arguments and return a {@link TaraskStep},
 * for example {@link resolveSpecialSyntax}.
 *
 * @module
 */

export * from './convert-alphabet';
export * from './highlight-diff';
export * from './i-to-j';
export * from './join-splitted';
export * from './apply-g';
export * from './apply-variations';
export * from './prepare';
export * from './resolve-syntax';
export * from './restore-case';
export * from './store-splitted-abc-converted-orig';
export * from './store-splitted-text';
export * from './taraskevize';
export * from './phonetize';
export * from './iotacize-ji';
export * from './whitespaces';
export * from './trim';
export * from './finalize';
export * from './to-lower-case';
export * from './escape-left-angle-bracket';
export type * from './types';
