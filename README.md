If you're looking for JSON dictionaries, check out the github releases.

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
	pipelines,
	TaraskConfig,
	htmlConfigOptions,
	wrappers,
	alphabets,
	REPLACE_J,
	VARIATION,
} from 'taraskevizer';

pipelines.tarask('планета');
// "плянэта"

const cfg = new TaraskConfig({
	abc: alphabets.cyrillic,
	j: 'always',
	variations: 'first',
	wrappers: wrappers.ansiColor,
	g: true,
});
pipelines.tarask('планета і Гродна', cfg);
// "пл\x1b[32mя\x1b[0mн\x1b[32mэ\x1b[0mта \x1b[32mй\x1b[0m \x1b[35mГорадня\x1b[0m"

pipelines.tarask('энергія планеты', {
	...htmlConfigOptions,
	abc: alphabets.latin,
	g: false, // ignored, g matters for cyrillic alphabet only
});
// "en<tarF>erg</tarF>ija p<tarF>lan</tarF>ety"

const latinWithJiCfg = new TaraskConfig({
	abc: alphabets.latinJi,
});

pipelines.alphabetic('яна і іншыя', latinWithJiCfg);
// "jana j jinšyja"
```

# Builtin Pipelines

| Pipeline   | Taraskevization | Phonetization | Alphabet | Special syntax |
| ---------- | --------------- | ------------- | -------- | -------------- |
| tarask     | ✅              | ❌            | ✅       | ✅             |
| alphabetic | ❌              | ❌            | ✅       | ✅             |
| phonetic   | ❌              | ✅ (alpha)    | ✅       | ✅             |

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
