import generateJSON from './json-generator';

export default (text, extension) => {
	if (extension === 'js') generateJSON(text, true);
	return text.replace(/(?:\\u\S{4})+/g, ($0) => JSON.parse(`"${$0}"`));
};
