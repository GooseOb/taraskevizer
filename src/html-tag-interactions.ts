/**
 * Browser helper for the HTML output of the taraskevizer pipelines.
 *
 * Works on text converted with the `html` wrappers (see `htmlConfigOptions`):
 * `<tarH>` letter toggles and `<tarL data-l='…'>` variation cycling, with
 * change tracking via `changeList`. This module is DOM-only and dependency-free.
 */

interface ChangeableElement extends HTMLElement {
	seqNum: number;
}
type Subscriber = (changeList: number[]) => void;

/** `г↔ґ` swap for `<tarH>` toggles (mirror of the core `applyG` step). */
const gSwap: Record<string, string> = {
	г: 'ґ',
	Г: 'Ґ',
	ґ: 'г',
	Ґ: 'Г',
};

const applyG = (el: Element) => {
	el.textContent = gSwap[el.textContent as string] ?? el.textContent;
};

export const createInteractiveTags = ({
	variable = 'tarL',
	letterH = 'tarH',
	changeList = [],
}: Partial<
	Record<'variable' | 'letterH', string> & { changeList: number[] }
> = {}) => {
	variable = variable.toUpperCase();
	letterH = letterH.toUpperCase();

	const subscribers = new Set<Subscriber>();
	const notify = () => {
		for (const cb of subscribers) cb(changeList);
	};

	return {
		update: (root: Element) => {
			const elems = root.querySelectorAll<ChangeableElement>(
				`${letterH}, ${variable}`
			);
			if (changeList.length > elems.length) {
				changeList.length = elems.length;
			} else {
				while (changeList.length < elems.length) {
					changeList.push(0);
				}
				notify();
			}
			for (let i = 0; i < changeList.length; i++) {
				const el = elems[i];
				el.seqNum = i;
				if (changeList[i]) {
					switch (el.tagName) {
						case letterH:
							{
								applyG(el);
							}
							break;
						case variable: {
							let data = el.dataset.l!;
							if (data.includes(',')) {
								const dataArr = data.split(',');
								data = el.innerHTML;
								for (let j = 0; j < changeList[i]; ++j) {
									dataArr.push(data);
									data = dataArr.shift()!;
								}
								el.dataset.l = dataArr.join(',');
							} else {
								el.dataset.l = el.innerHTML;
							}
							el.innerHTML = data;
						}
					}
				}
			}
		},
		tryAlternate: (el: Element | ChangeableElement) => {
			if ('seqNum' in el) {
				switch (el.tagName) {
					case letterH:
						{
							changeList[el.seqNum] = changeList[el.seqNum] ? 0 : 1;
							applyG(el);
							notify();
						}
						break;
					case variable: {
						let data = el.dataset.l!;
						if (data.includes(',')) {
							const dataArr = data.split(',');
							dataArr.push(el.innerHTML);
							data = dataArr.shift()!;
							changeList[el.seqNum] =
								(changeList[el.seqNum] + 1) % (dataArr.length + 1);
							el.dataset.l = dataArr.join(',');
						} else {
							changeList[el.seqNum] = changeList[el.seqNum] ? 0 : 1;
							el.dataset.l = el.innerHTML;
						}
						el.innerHTML = data;
						notify();
					}
				}
			}
		},
		changeList,
		subscribe: (cb: Subscriber) => {
			subscribers.add(cb);
			return () => {
				subscribers.delete(cb);
			};
		},
	};
};
