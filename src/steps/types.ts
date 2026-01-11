import type { TaraskConfig } from '@/config';
import type { DeepReadonly } from '@/types';

/**
 * Step of a pipeline.
 */
export type TaraskStep<Storage extends object = object> = (context: {
	text: string;
	storage: Storage;
	cfg: DeepReadonly<TaraskConfig>;
}) => void;

/**
 * Step of an async pipeline.
 */
export type AsyncTaraskStep<Storage extends object = object> = (context: {
	text: string;
	storage: Storage;
	cfg: DeepReadonly<TaraskConfig>;
}) => void | Promise<void>;

export type SplittedTextStorage = {
	textArr: string[];
	origArr: readonly string[];
};
