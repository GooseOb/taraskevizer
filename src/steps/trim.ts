import { mutatingStep } from '@/lib';

/**
 * Created in {@link trim}.
 *
 * Used in {@link untrim}.
 */
export type TrimStorage = {
	trim: {
		before: string;
		after: string;
	};
};

/**
 * Captures whitespaces around the text.
 */
export const trim = mutatingStep<TrimStorage>(({ text, storage }) => {
	storage.trim = {
		before: /^\s*/.exec(text)![0],
		after: /\s*$/.exec(text)![0],
	};
	return ` ${text.trim()} `;
});

/**
 * Brings original whitespaces around the text back.
 */
export const untrim = mutatingStep<TrimStorage>(
	({
		text,
		storage: {
			trim: { before, after },
		},
	}) => before + text.trim() + after
);
