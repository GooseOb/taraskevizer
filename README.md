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
import { Taraskevizer, ALPHABET, J, VARIATION } from 'taraskevizer';

const taraskevizer = new Taraskevizer();
taraskevizer.convert('планета');
// "плянэта"

const taraskevizer = new Taraskevizer({
	general: {
		abc: ALPHABET.CYRILLIC,
		j: J.ALWAYS,
	},
	nonHtml: {
		ansiColors: true,
		variations: VARIATION.FIRST,
		h: false,
	},
});
taraskevizer.convert('планета і Гродна');
// "пл\x1b[32mя\x1b[0mн\x1b[32mэ\x1b[0mта \x1b[32mй\x1b[0m \x1b[35mГорадня\x1b[0m"

const taraskevizer = new Taraskevizer({
	general: {
		abc: ALPHABET.LATIN,
	},
	html: {
		g: false, // ignored, because alphabet is set to latin
	},
});
taraskevizer.convertToHtml('энергія планеты');
// "en<tarF>erg</tarF>ija p<tarF>lan</tarF>ety"

// properties can be rewritten after creating an object
taraskevizer.abc = ALPHABET.ARABIC;
taraskevizer.html.g = true;

const latinizerWithJi = new Taraskevizer({
	general: { abc: ALPHABET.LATIN_JI },
});

latinizerWithJi.convertAlphabetOnly('яна і іншыя');
// "jana ji jinšyja"
```

# Options

## general

Type: `object`

### abc

Type: `number`

Default value: `0`

| Value | Alphabet of output text |
| ----- | ----------------------- |
| 0     | cyrillic                |
| 1     | latin                   |
| 2     | arabic                  |
| 3     | latin with ji           |

### j

Type: `number`

Default value: `0`

| Value | When to replace `і`(`i`) by `й`(`j`) after vowels | Example                  |
| ----- | ------------------------------------------------- | ------------------------ |
|       |                                                   | `яна і ён`               |
| 0     | never                                             | `яна і ён`               |
| 1     | random                                            | `яна і ён` or `яна й ён` |
| 2     | always                                            | `яна й ён`               |

Has no effect with `LATIN_JI` alphabet.

### doEscapeCapitalized

Type: `boolean`

Default `true`

If set to false, may cause unwanted changes in acronyms.

### OVERRIDE_taraskevize

Type: `(text: string) => string`

Default value: internal function `taraskevize`

Can be overridden in order to make additional changes to the text.
This function usually uses private api via `__tarask__`

## html

### g

Type: `boolean`

Default value: `false`

Do replace `г`(`h`) by `ґ`(`g`) in cyrillic alphabet?

| Value | Example                                 |
| ----- | --------------------------------------- |
| true  | `<tarH>г</tarH>валт <tarH>Г</tarH>валт` |
| false | `<tarH>ґ</tarH>валт <tarH>Ґ</tarH>валт` |

## nonHtml

### ansiColors

Type: `boolean`

Default value: `false`

### h

Type: `boolean`

Default value: `false`

Do replace ґ(g) by г(h) in cyrillic alphabet?

| Value | Example     |
| ----- | ----------- |
| true  | Ґвалт ґвалт |
| false | Гвалт гвалт |

### variations

Type: `number`

Default value: `0`

| Value | Which variation is used if a part of word is variable | Example           |
| ----- | ----------------------------------------------------- | ----------------- |
|       |                                                       | Гродна            |
| 0     | main                                                  | Гродна            |
| 1     | first                                                 | Горадня           |
| 2     | all                                                   | (Гродна\|Горадня) |

# HTML tags

## tarF

Difference between an input and an output word.

```html
<tarF>this_part_of_word_is_fixed</tarF>

пл<tarF>я</tarF>н
```

## tarL

A part of a word wrapped in this tag is variable,
variations are mentioned in a `data-l` attribute
and are separated by commas

```html
<tarL data-l="variation2,variation3">variation1</tarL>

<tarL data-l="Горадня">Гродна</tarL>
```

## tarH

Can be replaced by `ґ`(`g`) letter. appears only if alphabet is cyrillic

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
tarask "планета"
```

## Options

```bash
# Alpabet
--latin (-l)
--arabic (-a)
# When to replace і(i) by й(j) after vowels
--jrandom (-jr)
--jalways (-ja)
# Replace ґ(g) by г(h) in cyrillic alphabet
--h (-h)
# Variations
--no-variations (-nv)
--first-variation-only (-fvo)
# Other
--not-escape-caps (-nec)
--no-color (-nc)
--html (-html)
--alphabet-only (-abc) # not working; WIP
```

# Known bugs

## Replacing `не` with `ня`

`Ня` should appear before a word where the first syllabe is stressed.
At the moment, there is no way to check exactly if it is stressed.
Algorithm makes some heuristics, but that's not enough to cover all cases.
