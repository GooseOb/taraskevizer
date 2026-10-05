---
editUrl: false
next: true
prev: true
title: "TaraskAlphabet"
---

> **TaraskAlphabet** = `"cyrillic"` \| `"latin"` \| `"latinJi"` \| `"arabic"`

Defined in: [config.ts:13](https://github.com/GooseOb/taraskevizer/blob/f961046d449e43071ffa7b9a32044940361ce274/src/config.ts#L13)

Conversion options for the taraskevizer pipelines.

This module owns the `TaraskConfig` class: a plain TypeScript class.
The Rust/WASM side only parses the resulting shape (a `TaraskConfig`
instance or a plain `TaraskOptions` object) via `Reflect`, it never
manages the config object itself.

No runtime validation here — the types are the contract. Invalid values
from untyped callers are rejected by the Rust parser when a pipeline runs.
