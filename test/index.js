import { print, test } from './lib.js';
import { tarask, taraskSync } from '../dist/index.js';

print('test', 'start', '35');

test('Taraskevization', taraskSync, [
	['жыццясцвярджальны план', 'жыцьцясьцьвярджальны плян'],
	['варэнне', 'варэньне'],
]);

test('Sync and async functions return the same result', taraskSync, [
	['жыццясцвярджальны план', await tarask('жыццясцвярджальны план')],
]);

print('test', 'all tests passed successfully', '35');
