export const general = [
	['Планета', 'Плянэта'],
	['<Планета>', '<Планета>'],
	['<.Планета>', 'Планета'],
	['<,Планета>', '<Плянэта>'],
	['<*Планета>', '<Планета>'],
	['<*.Планета>', 'Планета'],
	['<*,Планета>', '<Плянэта>'],
] as const;

export const latin = [
	['Планета', 'Planeta'],
	['<Планета>', '<Планета>'],
	['<*Планета>', '<Płanieta>'],
	['<.Планета>', 'Планета'],
	['<*.Планета>', 'Płanieta'],
	['<,Планета>', '<Planeta>'],
	['<*,Планета>', '<Planeta>'],
] as const;
