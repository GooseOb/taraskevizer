import type { TaraskConfig } from '@/config';
import type { DeepReadonly } from '@/types';

export type TaraskStep<Storage extends object = object> = (context: {
	text: string;
	storage: Storage;
	cfg: DeepReadonly<TaraskConfig>;
}) => void;

export type SplittedTextStorage = {
	textArr: string[];
	origArr: readonly string[];
};
