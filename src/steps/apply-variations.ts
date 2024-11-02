import { mutatingStep } from '@/lib';
import { defaultVariation } from '@/wrappers';

export const applyVariations = mutatingStep(
	({ text, cfg: { wrappers, variations } }) =>
		text.replace(
			/\([^)]*?\)/g,
			(wrappers?.variable || defaultVariation)[variations]
		)
);
