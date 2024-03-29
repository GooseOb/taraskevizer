const notToChange = ['Cлова', 'CловA', 'CЛОВА', 'СлОвА'].map(
	(input) => [input, input] as const
);

export const escCap = [
	['ПЛАНЕТА', 'ПЛАНЕТА'],
	['планета', 'плянэта'],
	['Планета', 'Плянэта'],
	['ПлАНеТа', 'ПлАНеТа'],
	['ПланетА', 'ПЛЯНЭТА'],

	...notToChange,
] as const;

export const noEscCap = [
	['ПЛАНЕТА', 'ПЛЯНЭТА'],
	['планета', 'плянэта'],
	['Планета', 'Плянэта'],
	['ПлАНеТа', 'Плянэта'],
	['ПланетА', 'ПЛЯНЭТА'],

	...notToChange,
] as const;
