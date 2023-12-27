export const replaceWithDict = (text, dict, regex) => {
	for (const item of dict) {
		const [pattern, result] = item;
		text = text.replace(
			pattern,
			//@ts-ignore
			result
		);
		if (regex.test(text)) {
			log('replaceWithDict:', item);
			process.exit(1);
		}
	}
	return text;
};
export const log = (...msgs) => {
	console.log('[debug]', ...msgs);
};
