import { mutatingStep } from '../lib/index';
import { defaultVariationWrappers } from '../wrappers';

export const applyVariations = mutatingStep(
	({ text, cfg: { wrapperDict, variations } }) =>
		text.replace(
			/\([^)]*?\)/g,
			(wrapperDict?.variable || defaultVariationWrappers)[variations]
		)
);
