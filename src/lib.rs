//! # weighted_levenshtein
//!
//! A generic implementation of [Levenshtein distance](http://en.wikipedia.org/wiki/Levenshtein_distance)
//! that supports arbitrary weighting of edit operations.
//!
//! # Optional Features
//!
//! - `default-weight` (requires `feature(specialization)`, so nightly Rust): Provides a default
//!   implementation of `EditWeight` for any `PartialEq` type.  All operations have weight `1`.
//!
#![cfg_attr(feature = "default-weight", feature(specialization))]
#![allow(incomplete_features)]
use std::cmp::min;
use std::mem::swap;

/// Trait for types that have custom costs for addition/removal and
/// substitution.
pub trait EditWeight {
    /// Cost of adding this item to a sequence
    fn add_cost(&self) -> usize;
    /// Cost of removing this item from a sequence
    fn rm_cost(&self) -> usize;
    /// Cost of substituting `other` for this item.
    /// Note: edit distance is only well-defined when
    /// `self.sub_cost(&self) == 0`
    fn sub_cost(&self, other: &Self) -> usize;
}

/// Default implementation for all `PartialEq` types. Requires `default-weight` feature.
#[cfg(feature = "default-weight")]
use std::cmp::max;
#[cfg(feature = "default-weight")]
impl<T: PartialEq> EditWeight for T {
    default fn add_cost(&self) -> usize {
        1
    }
    default fn rm_cost(&self) -> usize {
        1
    }
    default fn sub_cost(&self, other: &Self) -> usize {
        if self == other {
            0
        } else {
            max(self.rm_cost(), other.add_cost())
        }
    }
}

// some specific impls otherwise
impl EditWeight for u8 {
    fn add_cost(&self) -> usize {
        1
    }
    fn rm_cost(&self) -> usize {
        1
    }
    fn sub_cost(&self, other: &Self) -> usize {
        if self == other {
            0
        } else {
            1
        }
    }
}
impl EditWeight for &str {
    fn add_cost(&self) -> usize {
        1
    }
    fn rm_cost(&self) -> usize {
        1
    }
    fn sub_cost(&self, other: &Self) -> usize {
        if self == other {
            0
        } else {
            1
        }
    }
}

/// Compute the Levenshtein distance between two sequences.
///
/// # Examples:
///
/// - Compute edit distance between strings, assuming ASCII or Latin-1.
///
/// ```rust
/// # use weighted_levenshtein::distance;
/// # fn main() {
/// assert_eq!(distance("abc", "aaxcc"), 3);
/// # }
/// ```
///
/// - Compute edit distance between strings, respecting Unicode.
///
/// ```rust
/// # use weighted_levenshtein::distance;
/// use unicode_segmentation::UnicodeSegmentation;
/// # fn main() {
/// assert_eq!(
///      distance(
///         "🇬🇧 🇧🇬".graphemes(true).collect::<Vec<&str>>(),
///         "🇬🇧 🏴󠁧󠁢󠁳󠁣󠁴󠁿".graphemes(true).collect::<Vec<&str>>()),
///      1);
/// # }
///
/// ```
/// - Compute a distance in words between two strings:
///
/// ```rust
/// # use weighted_levenshtein::distance;
/// # fn main() {
/// assert_eq!(
///    distance(
///       "The quick brown fox".split (' ').collect::<Vec<&str>>(),
///       "The very quick brown cat".split (' ').collect::<Vec<&str>>()),
///    2);
/// # }
/// ```
///
/// - Compute a distance between two sequences of a user type:
///
/// ```rust
/// # use weighted_levenshtein::{distance, EditWeight};
///
/// # fn main() {
/// assert_eq!(distance(vec![1, 2, 3], vec![0, 1, 3, 3, 4]), 3);
/// # }
/// ```
pub fn distance<T, U, V>(a: U, b: V) -> usize
where
    T: PartialEq + EditWeight,
    U: AsRef<[T]>,
    V: AsRef<[T]>,
{
    let mut a = a.as_ref();
    let mut b = b.as_ref();

    if a == b {
        return 0;
    }
    if a.len() > b.len() {
        swap(&mut a, &mut b);
    }

    // initialize matrix
    let mut mat: Vec<Vec<usize>> = Vec::with_capacity(a.len() + 1);
    for _ in 0..=a.len() {
        let mut row = Vec::with_capacity(b.len() + 1);
        for _ in 0..=b.len() {
            row.push(0);
        }
        mat.push(row);
    }
    // set first row (`a` prefixes vs empty string)
    for i in 0..a.len() {
        mat[i + 1][0] = mat[i][0] + a[i].rm_cost();
    }
    // set first column (`b` prefixes vs empty string)
    for i in 0..b.len() {
        mat[0][i + 1] = mat[0][i] + b[i].add_cost();
    }
    // fill in matrix
    for j in 0..b.len() {
        for i in 0..a.len() {
            let sub_cost = a[i].sub_cost(&b[j]);

            mat[i + 1][j + 1] = min(
                min(
                    mat[i][j + 1] + a[i].rm_cost(),
                    mat[i + 1][j] + b[j].add_cost(),
                ),
                mat[i][j] + sub_cost,
            );
        }
    }
    // return bottom right corner
    *mat.last().unwrap().last().unwrap()
}

/********************************************************************
 * Tests
 *******************************************************************/
#[cfg(test)]
mod tests {
    use super::{distance, EditWeight};
    use std::cmp::{max, min};
    use unicode_segmentation::UnicodeSegmentation;

    #[test]
    fn identical_strings_should_have_zero_distance() {
        assert_eq!(distance("abc", "abc"), 0);
    }

    #[test]
    fn insertions_should_increase_the_distance() {
        assert_eq!(distance("abc", "abcc"), 1);
        assert_eq!(distance("abc", "aabc"), 1);
        assert_eq!(distance("abc", "abbc"), 1);
    }

    #[test]
    fn deletions_should_increase_the_distance() {
        assert_eq!(distance("abcd", "abc"), 1);
        assert_eq!(distance("aabc", "abc"), 1);
        assert_eq!(distance("abbc", "abc"), 1);
    }

    #[test]
    fn bug_insert_at_beginning_of_longest_sequence() {
        // In order to change the longest sequence into the shortest, we
        // must:
        //
        // - insert an item ("x" -> cost: 1),
        // - copy some items ("abc" -> cost: 0),
        // - remove the trailing items ("defg" -> cost 4).
        //
        // The first insertion was wrongly counted as zero-cost.
        assert_eq!(distance("abcdefg", "xabc"), 5);
    }

    #[test]
    fn substitutions_should_increase_the_distance() {
        assert_eq!(distance("abc", "xbc"), 1);
        assert_eq!(distance("abc", "axc"), 1);
        assert_eq!(distance("abc", "abx"), 1);
    }

    #[test]
    fn should_work_on_integer_slices() {
        assert_eq!(distance(vec![0, 1, 2], vec![0, 1, 2]), 0);
        assert_eq!(distance(vec![0, 1, 2], vec![0, 0, 1, 2]), 1);
        assert_eq!(distance(vec![0, 1, 2], vec![1, 2]), 1);
        assert_eq!(distance(vec![0, 1, 2], vec![3, 1, 2]), 1);
    }

    #[test]
    fn test_grapheme_distance() {
        assert_eq!(
            distance(
                "🇬🇧🇧🇬".graphemes(true).collect::<Vec<&str>>(),
                "🇬🇧🏴󠁧󠁢󠁳󠁣󠁴󠁿".graphemes(true).collect::<Vec<&str>>(),
            ),
            1
        );
    }

    #[test]
    fn should_work_on_sentences() {
        assert_eq!(
            distance(
                "The quick brown fox jumps over the lazy dog"
                    .split(' ')
                    .collect::<Vec<&str>>(),
                "The quick brown fox jumps over the lazy dog"
                    .split(' ')
                    .collect::<Vec<&str>>()
            ),
            0
        );
        assert_eq!(
            distance(
                "The quick brown fox jumps over the lazy dog"
                    .split(' ')
                    .collect::<Vec<&str>>(),
                "The quick brown fox jumps over the very lazy dog"
                    .split(' ')
                    .collect::<Vec<&str>>()
            ),
            1
        );
        assert_eq!(
            distance(
                "The quick brown fox jumps over the lazy dog"
                    .split(' ')
                    .collect::<Vec<&str>>(),
                "The brown fox jumps over the lazy dog"
                    .split(' ')
                    .collect::<Vec<&str>>()
            ),
            1
        );
        assert_eq!(
            distance(
                "The quick brown fox jumps over the lazy dog"
                    .split(' ')
                    .collect::<Vec<&str>>(),
                "The quick brown cat jumps over the lazy dog"
                    .split(' ')
                    .collect::<Vec<&str>>()
            ),
            1
        );
    }

    #[test]
    fn shortcut_cases() {
        assert_eq!(distance("", "foo"), 3);
        assert_eq!(distance("a", "foo"), 3);
        assert_eq!(distance("o", "foo"), 2);
    }

    #[derive(PartialEq, Debug)]
    enum Lett {
        A,
        B,
        C,
        D,
    }
    #[cfg(not(feature = "default-weight"))]
    impl EditWeight for Lett {
        fn add_cost(&self) -> usize {
            1
        }
        fn rm_cost(&self) -> usize {
            1
        }
        fn sub_cost(&self, other: &Self) -> usize {
            if self == other {
                0
            } else {
                1
            }
        }
    }

    #[test]
    fn should_work_on_enum_slices() {
        assert_eq!(
            distance(&[Lett::B, Lett::A, Lett::D], &[Lett::C, Lett::A, Lett::B]),
            2
        );
    }

    #[derive(PartialEq)]
    enum Money {
        Nickel,
        Quarter,
        Dollar,
    }

    impl EditWeight for Money {
        fn add_cost(&self) -> usize {
            match self {
                Money::Nickel => 5,
                Money::Quarter => 25,
                Money::Dollar => 100,
            }
        }
        fn rm_cost(&self) -> usize {
            self.add_cost()
        }
        fn sub_cost(&self, other: &Self) -> usize {
            max(self.add_cost(), other.add_cost()) - min(self.add_cost(), other.add_cost())
        }
    }

    #[test]
    fn test_money() {
        assert_eq!(
            distance(
                &[Money::Dollar, Money::Nickel, Money::Dollar],
                &[Money::Dollar, Money::Nickel, Money::Quarter]
            ),
            75
        );
    }

    #[derive(PartialEq, Debug)]
    enum P {
        A,
        B,
        C,
        E,
    }

    impl EditWeight for P {
        fn add_cost(&self) -> usize {
            match self {
                P::C => 2,
                P::E => 3,
                _ => 1,
            }
        }
        fn rm_cost(&self) -> usize {
            self.add_cost()
        }
        #[cfg(not(feature = "default-weight"))]
        fn sub_cost(&self, other: &Self) -> usize {
            if self == other {
                0
            } else {
                std::cmp::max(self.add_cost(), other.add_cost())
            }
        }
    }

    #[test]
    fn test_expensive_subst() {
        assert_eq!(distance(&[P::B, P::A, P::B], &[P::B, P::A, P::E]), 3);
    }

    #[test]
    fn test_complex() {
        assert_eq!(distance(&[P::B, P::E, P::C, P::E], &[P::B, P::C, P::E]), 3);
    }

    #[test]
    fn test_cheap_delete() {
        assert_eq!(distance(&[P::B, P::E], &[P::B, P::A, P::E]), 1);
    }
}
