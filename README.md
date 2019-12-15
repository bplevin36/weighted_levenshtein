# README #

[![Docs](https://docs.rs/generic_levenshtein/badge.svg)](https://docs.rs/generic_levenshtein/)

A   generic    and   fast    implementation   of    the   [Levenshtein
distance](http://en.wikipedia.org/wiki/Levenshtein_distance).

## Generic ##

This crate can work on slices of any kind. It can therefore:

- Compute a distance in characters between two strings:

```rust
assert_eq!(distance ("abc", "aaxcc"), 3);
```

- Compute a distance in words between two strings:

```rust
assert_eq!(
   distance (
      "The quick brown fox".split (' ').collect::<Vec<_>>(),
      "The very quick brown cat".split (' ').collect()),
   2);
```

- Or even compute a distance between two sequences of anything:

```rust
assert_eq!(distance (vec![1, 2, 3], vec![0, 1, 3, 3, 4]), 3);
```

### Fast ###

At the  time of writing, this  crate is the fastest  on crates.io when
working with text (on par with `eddie`):

|                                                            | Identical | Same length | Different lengths |
|:-----------------------------------------------------------|----------:|------------:|------------------:|
| generic_levenshtein                                        |     **3** |     9_616   |        **11_010** |
| [levenshtein](https://crates.io/crates/levenshtein) v1.0.4 |       4   |    11_030   |          12_777   |
| [strsim](https://crates.io/crates/strsim) v0.9.2           |   9_173   |    10_100   |          11_738   |
| [distance](https://crates.io/crates/distance) v0.4.0       |  23_594   |    13_800   |          26_890   |
| [eddie](https://crates.io/crates/eddie) v0.3.2             |     215   |   **7_464** |          11_968   |
| [txtdist](https://crates.io/crates/txtdist) v0.2.1         |  17_732   |    17_635   |          20_975   |

All times in `ns` on an `Intel(R) Core(TM) i5-4300M CPU @ 2.60GHz`.
