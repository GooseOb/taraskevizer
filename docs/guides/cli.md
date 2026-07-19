---
title: CLI
description: Use taraskevizer from the command line.
sidebar:
  order: 7
---

The package installs a `tarask` binary. It wraps the `tarask` pipeline with
flags that map onto [`TaraskConfig`](/taraskevizer/reference/classes/taraskconfig/)
options.

## Install

```sh
npm install -g taraskevizer
# or
yarn global add taraskevizer
# or
bun add -g taraskevizer
```

## Usage

```sh
tarask [options] text
```

For the full option list, run `tarask --help`.

### Without installation

```sh
npx taraskevizer [options] text
# or
bunx taraskevizer [options] text
```

## Examples

Convert and latinize a word:

```sh
tarask --latin 'планета'
# p{\fix l}a{\fix ne}ta
```

Read from a file and write the converted text to another:

```sh
tarask < ./cyr-text.txt > ./lat-text.txt
```

Interactive mode (no text argument) — prompts for input line by line:

```sh
tarask
# Enter the text:
```

## Options

### General

| Flag              | Description  |
| ----------------- | ------------ |
| `-h`, `--help`    | Show help    |
| `-v`, `--version` | Show version |

### Alphabet

| Flag                | Description                     |
| ------------------- | ------------------------------- |
| `-l`, `--latin`     | Use the Latin alphabet          |
| `-lj`, `--latin-ji` | Use the Latin alphabet with і/ј |
| `-a`, `--arabic`    | Use the Arabic alphabet         |

### When to replace і(i) by й(j) after vowels

| Flag               | Description |
| ------------------ | ----------- |
| `-jr`, `--jrandom` | Random      |
| `-ja`, `--jalways` | Always      |

### Replace ґ(g) by г(h) in the Cyrillic alphabet

| Flag  | Description        |
| ----- | ------------------ |
| `--h` | Use г instead of ґ |

### Variations

| Flag                       | Description             |
| -------------------------- | ----------------------- |
| `-nv`, `--no-variations`   | Use the main spelling   |
| `-fv`, `--first-variation` | Use the first variation |

### Mode (only one may be used)

| Flag                      | Description                                |
| ------------------------- | ------------------------------------------ |
| `-html`, `--html`         | HTML mode (emit `tarF`/`tarL`/`tarH` tags) |
| `-abc`, `--alphabet-only` | Alphabet-only conversion                   |

### Other

| Flag                        | Description                     |
| --------------------------- | ------------------------------- |
| `-nec`, `--not-escape-caps` | Don't protect capitalized runs  |
| `-nc`, `--no-color`         | Disable ANSI color wrapping     |
| `-st`, `--single-thread`    | Force single-threaded execution |

## Known bugs

### Replacing `не` with `ня`

`Ня` should appear before a word where the first syllable is stressed.
At the moment there is no reliable way to check whether it is stressed.
The algorithm applies heuristics, but they do not cover all cases.
