import { callableDict } from '@/dict/lib';

export const endZSoftenAndNiaBiaz = callableDict([
	// / не (?=
	//   {зычны}*{сымбаль}{націск}
	//  |{зычны}*{галосны}{зычны}*{канец слова}
	// )
	// (?!{прыназоўнік})/
	[
		/ не (?=[бвгджзйклмнпхрстфхцчшўьʼ]*.\u0301|[бвгджзйклмнпхрстфхцчшўьʼ]*[аеёіоуыэюя][бвгджзйклмнпхрстфхцчшўьʼ]* )(?!а[бд]?|б[ея]зь?|[дз]а|д?ля|дзеля|[нп]ад?|пр[аы]|празь?|у|церазь?)/g,
		' ня ',
	],
	// / без(?=ь? (?:
	//   {зычны}*{сымбаль}{націск}
	//  |{зычны}*{галосны}{зычны}*{канец слова}
	// )/
	[
		/ без(?=ь? (?:[бвгджзйклмнпхрстфхцчшўьʼ]*.\u0301|[бвгджзйклмнпхрстфхцчшўьʼ]*[аеёіоуыэюя][бвгджзйклмнпхрстфхцчшўьʼ]* ))/g,
		' бяз',
	],

	/*
	 * @example "без імглаў" -> "бязь імглаў"
	 */
	[/ б[ея]з(?= і\S*[ая]ў|ну )/g, ' бязь'],
	[/ (?:пра|цера)?з(?= і\S*[ая]ў|ну )/g, '$&ь'],
]);
