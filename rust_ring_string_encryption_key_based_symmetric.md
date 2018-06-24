---
title: Rust symmetric string encryption using a key file and ring
keywords: example template
summary: "Symmetric string encryption in Rust with key generation"
permalink: rust_ring_string_encryption_key_based_symmetric.html
folder: Rust ring
references: [
    {
        url: "https://briansmith.org/rustdoc/ring/aead/index.html",
        description: "ring::aead documentation"
    },
    {
        url: "https://briansmith.org/rustdoc/ring/rand/index.html",
        description: "ring::rand documentation"
    },
]
authors: [
    {
        name: "Philipp Keck",
        url: "https://github.com/Philipp91"
    }
]
# List all reviewers that reviewed this version of the example. When the example is updated all old reviews
# must be removed from the list below and the code has to be reviewed again. The complete review process
# is documented in the main repository of CryptoExamples
current_reviews: [
]
# Indicates when this example was last updated/created. Reviews don't change this.
last_updated: "2018-06-24"
tags: [Rust, ring, AEAD, AES, GCM]
---

## Use cases

- Encrypted data storage
- Encrypted communication channel

## Preparations

- Add the [ring](https://crates.io/crates/ring) dependency to your `Cargo.toml`.

## Sample code

```rust
{% include_relative src/string_encryption_key_based_symmetric.rs %}
```

{% include links.html %}
