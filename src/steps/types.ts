import { TaraskConfig } from '../config';

export type TaraskStep<Storage = {}> = (args: {
	text: string;
	storage: Storage;
	cfg: TaraskConfig;
}) => void;

export type SplittedTextStorage = {
	textArr: string[];
	origArr: readonly string[];
};
