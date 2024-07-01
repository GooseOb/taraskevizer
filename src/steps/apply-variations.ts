import { mutatingStep } from '../lib';
import { defaultVariationWrappers } from '../wrappers';

export const applyVariations = mutatingStep(
	({ text, cfg: { wrapperDict, variations } }) =>
		text.replace(
			/\([^)]*?\)/g,
			(wrapperDict?.variable || defaultVariationWrappers)[variations]
		)
);
