import { mutatingStep } from '../lib';

export const trim = mutatingStep(({ text }) => ` ${text.trim()} `);
