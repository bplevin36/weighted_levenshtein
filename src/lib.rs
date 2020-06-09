//! # weighted_levenshtein
//!
//! A generic implementation of [Levenshtein distance](http://en.wikipedia.org/wiki/Levenshtein_distance)
//! that supports arbitrary weighting of edit operations.
//! 
//! # Optional Features
//! 
//! - `default-weight` (requires building w/ nightly Rust): Provides a default 
//!   implementation of `EditWeight` for any `PartialEq` type.
//!
#![cfg_attr(feature = "default-weight", feature(specialization))]

use std::cmp::min;
use std::mem::swap;
use std::iter::once;

/// Trait for types that have custom costs for addition/removal and
/// substitution.
pub trait EditWeight {
   /// Cost of adding or removing this item from a sequence
   fn addrm_cost(&self) -> usize;
   /// Cost of substituting `other` for this item.
   /// Implementors note: edit distance is only well-defined when 
   /// `self.sub_cost(&self) == 0`
   fn sub_cost(&self, other: &Self) -> usize;
}

/// Default implementation for all `PartialEq` types. Requires `default-weight` feature.
#[cfg(feature = "default-weight")]
impl<T: PartialEq> EditWeight for T {
   default fn addrm_cost(&self) -> usize { 1 }
   default fn sub_cost(&self, other: &Self) -> usize {
      if self == other {
         0
      } else {
         std::cmp::max(self.addrm_cost(), other.addrm_cost())
      }
   }
}

// some specific impls otherwise
impl EditWeight for u8 {
   fn addrm_cost(&self) -> usize { 1 }
   fn sub_cost(&self, other: &Self) -> usize {
      if self == other { 0 } else { 1 }
   }
}
impl EditWeight for &str {
   fn addrm_cost(&self) -> usize { 1 }
   fn sub_cost(&self, other: &Self) -> usize { 
      if self == other { 0 } else { 1 }
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
/// assert_eq!(distance ("abc", "aaxcc"), 3);
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
///    distance (
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
/// assert_eq!(distance (vec![1, 2, 3], vec![0, 1, 3, 3, 4]), 3);
/// # }
/// ```
pub fn distance<T, U: AsRef<[T]>, V: AsRef<[T]>> (a: U, b: V) -> usize
   where T: PartialEq + EditWeight
{
   let mut a = a.as_ref();
   let mut b = b.as_ref();

   if a == b { return 0; }

   if a.len() > b.len() { swap (&mut a, &mut b); }

   if a.len() == 0 { return b.len(); }

   // init prev_row
   let mut prev_row: Vec<usize> =
      b.iter().chain(once(&b[0])).scan(0usize, |cum, i| {
         let old_cum = *cum;
         *cum += i.addrm_cost();
         Some(old_cum)
      }).collect();
   let mut curr_row: Vec<usize> = vec![0; prev_row.len()];
   // loop through rows in matrix
   for (i, a_elem) in a.iter().enumerate() {
      // edit distance from current a to empty b
      curr_row[0] = a.iter().take(i+1).map(|x| x.addrm_cost()).sum();

      for (j, b_elem) in b.iter().enumerate() {
         let deletion_cost = prev_row[j + 1] + b_elem.addrm_cost();
         let insertion_cost = curr_row[j] + b_elem.addrm_cost();
         let sub_cost = prev_row[j] + a_elem.sub_cost(&b_elem);
         curr_row[j + 1] = min(deletion_cost, min(insertion_cost, sub_cost));
      }
      swap(&mut curr_row, &mut prev_row);
   }
   return *prev_row.last().unwrap();
}

/********************************************************************
 * Tests
 *******************************************************************/
#[cfg(test)]
mod tests {
   use super::{distance, EditWeight};
   use unicode_segmentation::UnicodeSegmentation;
   use std::cmp::{max, min};

   #[test]
   fn identical_strings_should_have_zero_distance() {
      assert_eq!(distance ("abc", "abc"), 0);
   }

   #[test]
   fn insertions_should_increase_the_distance() {
      assert_eq!(distance ("abc", "abcc"), 1);
      assert_eq!(distance ("abc", "aabc"), 1);
      assert_eq!(distance ("abc", "abbc"), 1);
   }

   #[test]
   fn deletions_should_increase_the_distance() {
      assert_eq!(distance ("abcd", "abc"), 1);
      assert_eq!(distance ("aabc", "abc"), 1);
      assert_eq!(distance ("abbc", "abc"), 1);
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
      assert_eq!(distance ("abcdefg", "xabc"), 5);
   }

   #[test]
   fn substitutions_should_increase_the_distance() {
      assert_eq!(distance ("abc", "xbc"), 1);
      assert_eq!(distance ("abc", "axc"), 1);
      assert_eq!(distance ("abc", "abx"), 1);
   }

   #[test]
   fn should_work_on_integer_slices() {
      assert_eq!(distance (vec![0, 1, 2], vec![0, 1, 2]), 0);
      assert_eq!(distance (vec![0, 1, 2], vec![0, 0, 1, 2]), 1);
      assert_eq!(distance (vec![0, 1, 2], vec![1, 2]), 1);
      assert_eq!(distance (vec![0, 1, 2], vec![3, 1, 2]), 1);
   }

   #[test]
   fn test_grapheme_distance() {
      assert_eq!(
         distance(
            "🇬🇧🇧🇬".graphemes(true).collect::<Vec<&str>>(),
            "🇬🇧🏴󠁧󠁢󠁳󠁣󠁴󠁿".graphemes(true).collect::<Vec<&str>>(),
         ),
      1);
   }

   #[test]
   fn should_work_on_sentences() {
      assert_eq!(
         distance (
            "The quick brown fox jumps over the lazy dog".split (' ').collect::<Vec<&str>>(),
            "The quick brown fox jumps over the lazy dog".split (' ').collect::<Vec<&str>>()),
         0);
      assert_eq!(
         distance (
            "The quick brown fox jumps over the lazy dog".split (' ').collect::<Vec<&str>>(),
            "The quick brown fox jumps over the very lazy dog".split (' ').collect::<Vec<&str>>()),
         1);
      assert_eq!(
         distance (
            "The quick brown fox jumps over the lazy dog".split (' ').collect::<Vec<&str>>(),
            "The brown fox jumps over the lazy dog".split (' ').collect::<Vec<&str>>()),
         1);
      assert_eq!(
         distance (
            "The quick brown fox jumps over the lazy dog".split (' ').collect::<Vec<&str>>(),
            "The quick brown cat jumps over the lazy dog".split (' ').collect::<Vec<&str>>()),
         1);
   }

   #[test]
   fn shortcut_cases() {
       assert_eq!(distance("", "foo"), 3);
       assert_eq!(distance("a", "foo"), 3);
       assert_eq!(distance("o", "foo"), 2);
   }

   #[derive(PartialEq, Debug)]
   enum Lett {
       A, B, C, D
   }
   #[cfg(not(feature = "default-weight"))]
   impl EditWeight for Lett {
      fn addrm_cost(&self) -> usize { 1 }
      fn sub_cost(&self, other: &Self) -> usize { 
         if self == other { 0 } else { 1 }
      }
   }

   #[test]
   fn should_work_on_enum_slices() {
       assert_eq!(
           distance (
               &[Lett::B, Lett::A , Lett::D], &[Lett::C, Lett::A , Lett::B]
           ),
           2);
   }

   #[derive(PartialEq)]
   enum Money {
      Nickel,
      Quarter,
      Dollar
   }

   impl EditWeight for Money {
      fn addrm_cost(&self) -> usize {
         match self {
            Money::Nickel => 5,
            Money::Quarter => 25,
            Money::Dollar => 100,
         }
      }
      fn sub_cost(&self, other: &Self) -> usize {
         max(self.addrm_cost(), other.addrm_cost()) - min(self.addrm_cost(), other.addrm_cost())
      }
   }

   #[test]
   fn test_money() {
      assert_eq!(
         distance(
            &[Money::Dollar, Money::Nickel, Money::Dollar],
            &[Money::Dollar, Money::Nickel, Money::Quarter]),
         75);
   }


   #[derive(PartialEq, Debug)]
   enum P {
      A, B, C, E
   }

   impl EditWeight for P {
      fn addrm_cost(&self) -> usize {
         match self {
            P::C => 2,
            P::E => 3,
            _ => 1,
         }
      }
      #[cfg(not(feature = "default-weight"))]
      fn sub_cost(&self, other: &Self) -> usize {
         if self == other {
            0
         } else {
            std::cmp::max(self.addrm_cost(), other.addrm_cost())
         }
      }
   }

   #[test]
   fn test_expensive_subst() {
      assert_eq!(distance(&[P::B, P::A, P::B], &[P::B, P::A, P::E]), 3);
   }

   #[test]
   fn test_complex() {
      assert_eq!(distance(
         &[P::B, P::E, P::C, P::E],
         &[P::B, P::C, P::E]),
      3);
   }

   #[test]
   fn test_cheap_delete() {
      assert_eq!(distance(&[P::B, P::E], &[P::B, P::A, P::E]), 1);
   }
}
