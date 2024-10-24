import type { TaraskConfig } from '../config';

export type TaraskStep<Storage extends object = object> = (args: {
	text: string;
	storage: Storage;
	cfg: TaraskConfig;
}) => void;

export type SplittedTextStorage = {
	textArr: string[];
	origArr: readonly string[];
};
