[Full API documentation](https://gooseob.github.io/taraskevizer/)

# Install

With npm:

```bash
npm install taraskevizer
```

With yarn:

```bash
yarn add taraskevizer
```

With bun:

```bash
bun add taraskevizer
```

# Usage

```js
import {
	tarask,
	plainTextPipeline,
	htmlPipeline,
	abcOnlyPipeline,
	TaraskConfig,
	alphabets,
	J,
	VARIATION,
} from 'taraskevizer';

tarask('планета', plainTextPipeline);
// "плянэта"

const cfg = new TaraskConfig({
	general: {
		abc: alphabets.cyrillic,
		j: J.ALWAYS,
	},
	nonHtml: {
		ansiColors: true,
		variations: VARIATION.FIRST,
		h: false,
	},
});
tarask('планета і Гродна', plainTextPipeline, cfg);
// "пл\x1b[32mя\x1b[0mн\x1b[32mэ\x1b[0mта \x1b[32mй\x1b[0m \x1b[35mГорадня\x1b[0m"

const cfg = new TaraskConfig({
	general: {
		abc: alphabets.latin,
	},
	html: {
		g: false, // ignored, because alphabet is set to latin
	},
});
tarask('энергія планеты', htmlPipeline, cfg);
// "en<tarF>erg</tarF>ija p<tarF>lan</tarF>ety"

const latinWithJiCfg = new TaraskConfig({
	general: { abc: alphabets.latinJi },
});

tarask('яна і іншыя', abcOnlyPipeline, latinWithJiCfg);
// "jana j jinšyja"
```

# HTML tags

## tarF

Difference between the input and the output word.

```html
<tarF>this_part_of_word_is_fixed</tarF>

пл<tarF>я</tarF>н
```

## tarL

A part of a word wrapped in this tag is variable,
variations are mentioned in a `data-l` attribute,
separated with commas.

```html
<tarL data-l="variation2,variation3">variation1</tarL>

<tarL data-l="Горадня">Гродна</tarL>
```

## tarH

May be toggled between `г`(`h`) and `ґ`(`g`).
Appears only if alphabet is cyrillic.

```html
<tarH>г</tarH>

<tarH>Г</tarH>валт
```

# Special Syntax

|             | fix          | no fix       | change only alphabet |
| ----------- | ------------ | ------------ | -------------------- |
| brackets    | `<,Планета>` | `<Планета>`  | `<*Планета>`         |
| no brackets | `Планета`    | `<.Планета>` | `<*.Планета>`        |

# CLI

## Install

With npm:

```bash
npm install -g taraskevizer
```

With yarn:

```bash
yarn global add taraskevizer
```

With bun:

```bash
bun add -g taraskevizer
```

## Usage

```bash
tarask [options] text
```

For usage examples and options use `--help` option
(in source, content of `--help` is in [this file](./cli-help.txt))

### "Without installation"

With npm:

```bash
npx taraskevizer [options] text
```

With bun:

```bash
bunx taraskevizer [options] text
```

# Known bugs

## Replacing `не` with `ня`

`Ня` should appear before a word where the first syllabe is stressed.
At the moment, there is no way to check exactly if it is stressed.
Algorithm makes some heuristics, but that's not enough to cover all cases.
