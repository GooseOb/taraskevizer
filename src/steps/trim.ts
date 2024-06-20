import { mutatingStep } from '../lib/index';

type TrimStorage = {
	beforeTrim: string;
	afterTrim: string;
};

export const trim = mutatingStep<TrimStorage>(({ text, storage }) => {
	storage.beforeTrim = /^\s*/.exec(text)![0];
	storage.afterTrim = /\s*$/.exec(text)![0];
	return ` ${text.trim()} `;
});

export const untrim = mutatingStep<TrimStorage>(
	({ text, storage }) => storage.beforeTrim + text.trim() + storage.afterTrim
);
