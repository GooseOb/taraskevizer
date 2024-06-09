import { readableStreamToText } from 'bun';
import { watch } from 'fs/promises';
import * as path from 'path';

const pattern = /^(?:bin\/|src\/|test\/(?:index\.ts|lib\.ts|cases|large-text))/;

const cooldown = {
	value: true,
	set(time: number) {
		this.value = false;
		setTimeout(() => {
			this.value = true;
		}, time);
	},
};

let cursorPos = 0;

const update = async () => {
	cooldown.set(100);
	const text = await readableStreamToText(Bun.spawn(['bun', './test']).stdout);
	process.stdout.moveCursor(0, -cursorPos);
	process.stdout.clearScreenDown();
	process.stdout.write(text);
	cursorPos = text.match(/\n/g)?.length ?? 0;
};

await update();

const watcher = watch(path.resolve(import.meta.dir, '..'), {
	recursive: true,
});

for await (const event of watcher)
	if (
		event.filename &&
		cooldown.value &&
		event.filename.slice(-1) !== '~' &&
		pattern.test(event.filename)
	)
		await update();
