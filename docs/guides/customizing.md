---
title: Customizing
description: What can be tuned beyond the defaults.
sidebar:
  order: 6
---

The conversion core is compiled from Rust to WebAssembly, so the pipelines
are fixed entry points: custom steps, alphabets, wrappers, and dictionaries
cannot be composed from JavaScript. Everything tunable is covered by the
following.

## Configuration

[`TaraskConfig`](/taraskevizer/reference/classes/taraskconfig/) (or a plain
option object) tunes the alphabet, letter replacements, variations, and how
changed parts are wrapped. See
[Configuration](/taraskevizer/guides/configuration/) for every option.

## Special syntax

Inline markers control the conversion of individual words directly in the
input text, without a custom config. See
[Special syntax](/taraskevizer/guides/special-syntax/).

## CLI flags

Every config option is also a `tarask` flag, plus HTML output and
single-threaded modes. See [CLI](/taraskevizer/guides/cli/).

## Interactive HTML tags

Converting with the HTML wrappers produces `tarF`/`tarL`/`tarH` tags, and
[`createInteractiveTags`](/taraskevizer/reference/functions/createinteractivetags/)
turns them into clickable variant toggles in the browser. See
[HTML tags](/taraskevizer/guides/html-tags/).

## Deeper changes

New alphabets, pipeline steps, or dictionary entries live in the Rust core
(`taraskevizer-rs/`) and take effect everywhere — JS API, browser bundle,
and native CLI — once compiled. Contributions there are welcome.
