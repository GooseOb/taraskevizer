import { mutatingStep } from '@/lib';

export const toLowerCase = mutatingStep(({ text }) => text.toLowerCase());
