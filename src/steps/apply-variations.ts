import { mutatingStep } from '@/lib';
import { defaultVariation } from '@/wrappers';

export const applyVariations = mutatingStep(
	({ text, cfg: { wrapperDict, variations } }) =>
		text.replace(
			/\([^)]*?\)/g,
			(wrapperDict?.variable || defaultVariation)[variations]
		)
);
