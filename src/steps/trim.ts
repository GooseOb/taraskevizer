import { mutatingStep } from '../lib/index';

export const trim = mutatingStep(({ text }) => ` ${text.trim()} `);
