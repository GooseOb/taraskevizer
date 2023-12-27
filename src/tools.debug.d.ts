import type { ReplaceWithDict } from './types';
type AddParameter<TFn, TNewArg> = TFn extends (
	...args: infer TArgs
) => infer TReturn
	? (...args: [...TArgs, TNewArg]) => TReturn
	: never;
export declare const replaceWithDict: AddParameter<ReplaceWithDict, RegExp>;
export declare const log: (...msgs: any[]) => void;
export {};
