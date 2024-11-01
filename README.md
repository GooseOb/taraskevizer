[Go to full API documentation](https://gooseob.github.io/taraskevizer/)

# Install

With npm:

```sh
npm install taraskevizer
```

With yarn:

```sh
yarn add taraskevizer
```

With bun:

```sh
bun add taraskevizer
```

# Usage

```js
import {
	tarask,
	pipelines,
	TaraskConfig,
	htmlConfigOptions,
	ansiColorWrappers,
	alphabets,
	REPLACE_J,
	VARIATION,
} from 'taraskevizer';

tarask('планета', pipelines.tar);
// "плянэта"

const cfg = new TaraskConfig({
	abc: alphabets.cyrillic,
	j: 'always',
	variations: 'first',
	wrapperDict: ansiColorWrappers,
	g: true,
});
tarask('планета і Гродна', pipelines.tar, cfg);
// "пл\x1b[32mя\x1b[0mн\x1b[32mэ\x1b[0mта \x1b[32mй\x1b[0m \x1b[35mГорадня\x1b[0m"

const htmlCfg = new TaraskConfig({
	...htmlConfigOptions,
	abc: alphabets.latin,
	g: false, // ignored, g matters for cyrillic alphabet only
});
tarask('энергія планеты', pipelines.tar, htmlCfg);
// "en<tarF>erg</tarF>ija p<tarF>lan</tarF>ety"

const latinWithJiCfg = new TaraskConfig({
	abc: alphabets.latinJi,
});

tarask('яна і іншыя', pipelines.abcOnly, latinWithJiCfg);
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

```sh
npm install -g taraskevizer
```

With yarn:

```sh
yarn global add taraskevizer
```

With bun:

```sh
bun add -g taraskevizer
```

## Usage

```sh
tarask [options] text
```

For usage examples and options use `--help` option
(in source, content of `--help` is in [this file](./cli-help.txt))

### "Without installation"

With npm:

```sh
npx taraskevizer [options] text
```

With bun:

```sh
bunx taraskevizer [options] text
```

# Known bugs

## Replacing `не` with `ня`

`Ня` should appear before a word where the first syllabe is stressed.
At the moment, there is no way to check exactly if it is stressed.
Algorithm makes some heuristics, but that's not enough to cover all cases.
