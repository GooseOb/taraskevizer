import { mutatingStep } from '../lib';

/**
 * Created in {@link trim}.
 *
 * Used in {@link untrim}.
 */
export type TrimStorage = {
	beforeTrim: string;
	afterTrim: string;
};

/**
 * Captures whitespaces around the text.
 */
export const trim = mutatingStep<TrimStorage>(({ text, storage }) => {
	storage.beforeTrim = /^\s*/.exec(text)![0];
	storage.afterTrim = /\s*$/.exec(text)![0];
	return ` ${text.trim()} `;
});

/**
 * Brings original whitespaces around the text back.
 */
export const untrim = mutatingStep<TrimStorage>(
	({ text, storage }) => storage.beforeTrim + text.trim() + storage.afterTrim
);
