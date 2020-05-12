//! A   generic   and   fast  implementation   of   the   [Levenshtein
//! distance](http://en.wikipedia.org/wiki/Levenshtein_distance).

use std::cmp::min;
use std::mem::swap;

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
///       "The quick brown fox".split (' ').collect::<Vec<_>>(),
///       "The very quick brown cat".split (' ').collect()),
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
pub fn distance<T: PartialEq, U: AsRef<[T]>> (a: U, b: U) -> usize
{
   let mut a = a.as_ref();
   let mut b = b.as_ref();

   if a == b { return 0; }

   if a.len() > b.len() { swap (&mut a, &mut b); }

   if a.len() == 0 { return b.len(); }

   if a.len() == 1 {
       return if b.contains(&a[0]) {
           b.len() - 1
       } else {
           b.len() + 1
       }
   }

   let mut cache: Vec<_> = (1 ..= a.len()).map (|x| x * 1).collect();

   let mut result = 0;
   for (i, bi) in b.iter().enumerate() {
      result = (i+1) * 1;
      let mut up = i * 1;
      for (aj, c) in a.iter().zip (cache.iter_mut()) {
         let diag = if bi == aj { up } else { up + 1 };
         up = *c;
         result = min (min (result + 1,
                            up + 1),
                       diag);
         *c = result;
      }
   }

   return result;
}

/********************************************************************
 * Tests
 *******************************************************************/
#[cfg (test)]
mod tests {
   use super::distance;

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
            "The quick brown fox jumps over the lazy dog".split (' ').collect::<Vec<_>>(),
            "The quick brown fox jumps over the lazy dog".split (' ').collect::<Vec<_>>()),
         0);
      assert_eq!(
         distance (
            "The quick brown fox jumps over the lazy dog".split (' ').collect::<Vec<_>>(),
            "The quick brown fox jumps over the very lazy dog".split (' ').collect::<Vec<_>>()),
         1);
      assert_eq!(
         distance (
            "The quick brown fox jumps over the lazy dog".split (' ').collect::<Vec<_>>(),
            "The brown fox jumps over the lazy dog".split (' ').collect::<Vec<_>>()),
         1);
      assert_eq!(
         distance (
            "The quick brown fox jumps over the lazy dog".split (' ').collect::<Vec<_>>(),
            "The quick brown cat jumps over the lazy dog".split (' ').collect::<Vec<_>>()),
         1);
   }

   #[test]
   fn shortcut_cases() {
       assert_eq!(distance("", "foo"), 3);
       assert_eq!(distance("a", "foo"), 4);
       assert_eq!(distance("f", "foo"), 2);
   }

   #[derive(PartialEq)]
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
}
