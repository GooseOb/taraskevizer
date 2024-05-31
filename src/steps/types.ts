import { TaraskConfig } from '../config';

export type TaraskStep<Storage = {}> = (
	text: string,
	args: {
		storage: Storage;
		cfg: TaraskConfig;
	}
) => string;

export type SplittedTextStorage = {
	text: string[];
	orig: readonly string[];
};
