import { mutatingStep } from '../lib/index';

export const toLowerCase = mutatingStep(({ text }) => text.toLowerCase());
