# README #

[![Docs](https://docs.rs/weighted_levenshtein/badge.svg)](https://docs.rs/weighted_levenshtein/)

A   generic   implementation   of    the   [Levenshtein
distance](http://en.wikipedia.org/wiki/Levenshtein_distance) that allows arbitrarily
weighting operations for different elements.

## Generic ##

This crate can work on slices of any kind. It can:

- Compute a distance in characters between two strings:

```rust
assert_eq!(distance("abc", "aaxcc"), 3);
```

- Compute a distance in words between two strings:

```rust
assert_eq!(
   distance(
      "The quick brown fox".split (' ').collect::<Vec<_>>(),
      "The very quick brown cat".split (' ').collect()),
   2);
```

- Or compute a distance between arbitrary sequences:

```rust
assert_eq!(distance(vec![1, 2, 3], vec![0, 1, 3, 3, 4]), 3);
```

### Weighting ###

This crate allows defining custom weights for each operation on each symbol.
These weights can be specified for custom types by implementing the [`EditWeight`](https://docs.rs/weighted_levenshtein/0.2.0/weighted_levenshtein/trait.EditWeight.html)
trait.
