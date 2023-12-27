## Install

```bash
$ npm i taraskevizer
```

## Usage

```js
import { tarask, taraskToHtml, ALPHABET, J, VARIATION } from 'taraskevizer';

const taraskedText = tarask('планета');
// плянэта

const taraskedText = tarask(
	'планета і Гродна',
	{
		abc: ALPHABET.CYRILLIC,
		j: J.ALWAYS,
	},
	{
		ansiColors: true,
		variations: VARIATION.FIRST,
		h: false,
	}
);
// 'пл\x1b[32mя\x1b[0mн\x1b[32mэ\x1b[0mта \x1b[32mй\x1b[0m \x1b[35mГорадня\x1b[0m'

const taraskedTextHtml = taraskToHtml(
	'энергія планеты',
	{
		abc: ALPHABET.LATIN,
	},
	{
		g: false, // ignored, because alphabet is set to latin
	}
);
// en<tarF>erg</tarF>ija p<tarF>lan</tarF>ety
```

### Function signatures are in [this file](./dist/index.d.ts) (not available if project is not built)

## TaraskOptions

Type: `object`

### abc

Type: `number`

Default value: `0`

Alphabet of output text:

```
0 = cyrillic
1 = latin
2 = arabic
3 = greek
```

### j

Type: `number`

Default value: `0`

When to replace `і`(`i`) by `й`(`j`) after vowels:

```
0 = never
1 = random
2 = always
```

### OVERRIDE_toTarask

Type:

```
(
    text: string,
    replaceWithDict: (
        text: string,
        dict?: [RegExp, string | ((...substrings: string[]) => string)][]
    ) => string,
    wordlist: [RegExp, string][],
    softers: [RegExp, string][],
    afterTarask: (text: string) => string
) => string
```

Default value: internal function `toTarask`

## HtmlOptions

### g

Type: `boolean`

Default value: `false`

Do replace `г`(`h`) by `ґ`(`g`) in cyrillic alphabet?

```html
false: <tarH>г</tarH> <tarH>Г</tarH>

true: <tarH>ґ</tarH> <tarH>Ґ</tarH>
```

## NonHtmlOptions

### ansiColors

Type: `boolean`

Default value: `false`

### h

Type: `boolean`

Default value: `false`

Do replace ґ(g) by г(h) in cyrillic alphabet?

```
false: Ґ ґ

true: Г г
```

### variations

Type: `number`

Default value: `0`

Which variation should be if a part of word is variable?

```
0 = main:   Гродна
1 = first:  Горадня
2 = all:    (Гродна|Горадня)
```

## HTML tags

### tarF

Difference between an input and an output word.

```html
<tarF>this_part_of_word_is_fixed</tarF>

пл<tarF>я</tarF>н
```

### tarL

A part of a word wrapped in this tag is variable,
variations are mentioned in a `data-l` attribute
and are separated by commas

```html
<tarL data-l="variation2,variation3">variation1</tarL>

<tarL data-l="Горадня">Гродна</tarL>
```

### tarH

Can be replaced by `ґ`(`g`) letter. appears only if alphabet is cyrillic

```html
<tarH>г</tarH>

<tarH>Г</tarH>валт
```

# CLI

## Install

```bash
$ npm i -g taraskevizer
```

## Usage

```bash
$ tarask "планета"
```

## Options

```bash
# Alpabet
--latin (-l)
--arabic (-a)
--greek (-gr)
# When to replace і(i) by й(j) after vowels
--jrandom (-jr)
--jalways (-ja)
# Replace ґ(g) by г(h) in cyrillic alphabet
--h (-h)
# Variations
--no-variations (-nv)
--first-variation-only (-fvo)
# Other
--no-color (-nc)
--html (-html) # options except alphabet and j will be ignored
```
