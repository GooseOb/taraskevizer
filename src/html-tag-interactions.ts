import { gobj } from './dict';

type ChangeableElement = HTMLElement & { seqNum: number };

const applyG = (el: ChangeableElement) => {
	el.textContent = gobj[el.textContent as keyof typeof gobj];
};

type NodeList<T extends Node> = T[] | NodeListOf<T>;

const changeList: number[] = [];

export const apply = (elems: NodeList<ChangeableElement>) => {
	if (changeList.length > elems.length) {
		changeList.length = elems.length;
	} else {
		while (changeList.length < elems.length) {
			changeList.push(0);
		}
	}
	for (let i = 0; i < changeList.length; i++) {
		const el = elems[i];
		el.seqNum = i;
		if (changeList[i]) {
			switch (el.tagName) {
				case 'TARH':
					{
						applyG(el);
					}
					break;
				case 'TARL': {
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
};

/**
 * @param el non-changeable elements are ignored
 **/
export const tryAlternate = (el: Element | ChangeableElement) => {
	if ('seqNum' in el) {
		switch (el.tagName) {
			case 'TARH':
				{
					changeList[el.seqNum] = changeList[el.seqNum] ? 0 : 1;
					applyG(el);
				}
				break;
			case 'TARL': {
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
			}
		}
	}
};
