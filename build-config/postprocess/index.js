export default (text) => {
	return text.replace(/(?:\\u\S{4})+/g, ($0) => JSON.parse(`"${$0}"`));
};
