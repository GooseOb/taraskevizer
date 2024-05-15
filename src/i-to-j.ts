const toJ = (shortU: '' | 'ў') => 'й ' + (shortU ? 'у' : '');

export const replaceIbyJ = (text: string, always = false) =>
	text.replace(
		/(?<=[аеёіоуыэюя] )і (ў?)/g,
		always
			? ($0, $1) => toJ($1)
			: ($0, $1) => (Math.random() >= 0.5 ? toJ($1) : $0)
	);
