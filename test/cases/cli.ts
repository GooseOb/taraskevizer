export default [
	[['-nc', 'планета'], 'плянэта'],
	[['-nc', 'ПЛАНЕТА'], 'ПЛАНЕТА'],
	[['-nc', '-nec', 'ПЛАНЕТА'], 'ПЛЯНЭТА'],
	[['-nc', 'гродна'], '(гродна|горадня)'],
	[['-nc', '-nv', 'гродна'], 'гродна'],
	[['-nc', '-fvo', 'гродна'], 'горадня'],
	[['-nc', 'яна і ён'], 'яна і ён'],
	[['-nc', '-ja', 'яна і ён'], 'яна й ён'],
	[['-nc', '-l', 'планета'], 'planeta'],
	[['-l', 'планета'], 'p\x1b[32ml\x1b[0ma\x1b[32mne\x1b[0mta'],
	[['-nc', 'энергія'], 'энэрґія'],
	[['-nc', '-h', 'энергія'], 'энэргія'],
	[['энергія'], 'эн\x1b[32mэ\x1b[0mр\x1b[32m\x1b[35mґ\x1b[0m\x1b[0mія'],
	[['-h', 'энергія'], 'эн\x1b[32mэ\x1b[0mр\x1b[32m\x1b[35mг\x1b[0m\x1b[0mія'],
	[['-html', 'энергія'], 'эн<tarF>э</tarF>р<tarF><tarH>ґ</tarH></tarF>ія'],
	[
		['-html', '-h', 'энергія'],
		'эн<tarF>э</tarF>р<tarF><tarH>г</tarH></tarF>ія',
	],
	[['-nc', '-abc', '-lj', 'яна і іншыя'], 'jana j jinšyja'],
	[['-nc', '-abc', '-l', 'планета'], 'płanieta'],
] satisfies [string[], string][];
