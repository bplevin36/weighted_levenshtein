#![feature(specialization)]
//! A   generic   and   fast  implementation   of   the   [Levenshtein
//! distance](http://en.wikipedia.org/wiki/Levenshtein_distance).

use std::cmp::{min, max};
use std::mem::swap;
use std::iter::once;

/// Compute  the  Levenshtein  distance between  two  sequences  using
/// identical weights for insertions/deletions and for substitutions.
///
/// # Examples:
///
/// - Compute a distance in characters between two strings:
///
/// ```rust
/// # use weighted_levenshtein::distance;
/// # fn main() {
/// assert_eq!(distance ("abc", "aaxcc"), 3);
/// # }
/// ```
///
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
/// - Compute a distance between two sequences of anything:
///
/// ```rust
/// # use weighted_levenshtein::distance;
/// # fn main() {
/// assert_eq!(distance (vec![1, 2, 3], vec![0, 1, 3, 3, 4]), 3);
/// # }
/// ```
pub fn distance<T: PartialEq + EditWeight, U: AsRef<[T]>, V: AsRef<[T]>> (a: U, b: V) -> usize
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

pub trait EditWeight {
   fn addrm_cost(&self) -> usize;
   fn sub_cost(&self, other: &Self) -> usize;
}

impl<T: PartialEq> EditWeight for T {
   default fn addrm_cost(&self) -> usize { 1 }
   default fn sub_cost(&self, other: &Self) -> usize {
      if self == other {
         0
      } else {
         max(self.addrm_cost(), other.addrm_cost())
      }
   }
}

/********************************************************************
 * Tests
 *******************************************************************/
#[cfg (test)]
mod tests {
   use super::{distance, EditWeight};

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
   #[test]
   fn should_work_on_enum_slices() {
       assert_eq!(
           distance (
               &[Lett::B, Lett::A , Lett::D], &[Lett::C, Lett::A , Lett::B]
           ),
           2);
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
