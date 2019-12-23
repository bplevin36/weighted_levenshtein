//! A   generic   and   fast  implementation   of   the   [Levenshtein
//! distance](http://en.wikipedia.org/wiki/Levenshtein_distance).

use std::cmp::min;
use std::mem::swap;

/// Holds custom weights for substitutions and insertion/deletion
///
/// # Examples
///
/// * One substitution is equivalent to an insertion and a deletion:
///
/// ```rust
/// # use generic_levenshtein::Levenshtein;
/// # fn main() {
/// let lev = Levenshtein::new (1, 2);
/// // Substitute 'x' for 'b' -> cost = 2
/// assert_eq!(lev.distance ("abc", "axc"), 2);
/// // Insert 'x' and delete 'c' -> cost = 1+1 = 2
/// assert_eq!(lev.distance ("abc", "xab"), 2);
/// # }
/// ```
///
/// * Have  substitutions  cost  slightly  more  than  insertions  and
///   deletions  (but  still  less  than a  deletion  followed  by  an
///   insertion):
///
/// ```rust
/// # use generic_levenshtein::Levenshtein;
/// # fn main() {
/// let lev = Levenshtein::new (2, 3);
/// // Substitute 'x' for 'b' -> cost = 3
/// assert_eq!(lev.distance ("abc", "axc"), 3);
/// // Insert 'x' and delete 'c' -> cost = 2+2 = 4
/// assert_eq!(lev.distance ("abc", "xab"), 4);
/// # }
/// ```
pub struct Levenshtein {
   addrm_weight: usize,
   subst_weight: usize,
}

impl Levenshtein {
   /// Create a `Levenshtein` instance holding custom weights.
   ///
   /// # Arguments
   ///
   /// * `addrm_weight` Weight to use for insertions and deletions.
   /// * `subst_weight` Weight to use for substitutions.
   pub fn new (addrm_weight: usize, subst_weight: usize)
      -> Self
   {
      assert!(addrm_weight > 0);
      assert!(subst_weight > 0);
      Levenshtein { addrm_weight, subst_weight }
   }

   /// Compute  the Levenshtein  distance between two  sequences using
   /// the stored custom weights.
   pub fn distance<T: PartialEq, U: AsRef<[T]>> (&self, a: U, b: U)
      -> usize
   {
      let mut a = a.as_ref();
      let mut b = b.as_ref();

      if a == b { return 0; }

      if a.len() > b.len() { swap (&mut a, &mut b); }

      if a.len() == 0 { return b.len(); }

      let mut cache: Vec<_> = (1 ..= a.len()).map (|x| x * self.addrm_weight).collect();

      let mut result = 0;
      for (i, bi) in b.iter().enumerate() {
         result = (i+1) * self.addrm_weight;
         let mut up = i * self.addrm_weight;
         for (aj, c) in a.iter().zip (cache.iter_mut()) {
            let diag = if bi == aj { up } else { up + self.subst_weight };
            up = *c;
            result = min (min (result + self.addrm_weight,
                               up + self.addrm_weight),
                          diag);
            *c = result;
         }
      }

      return result;
   }
}

impl Default for Levenshtein {
   fn default() -> Self {
      Levenshtein { addrm_weight: 1, subst_weight: 1 }
   }
}

/// Compute  the  Levenshtein  distance between  two  sequences  using
/// identical weights for insertions/deletions and for substitutions.
///
/// # Examples:
///
/// - Compute a distance in characters between two strings:
///
/// ```rust
/// # use generic_levenshtein::distance;
/// # fn main() {
/// assert_eq!(distance ("abc", "aaxcc"), 3);
/// # }
/// ```
///
/// - Compute a distance in words between two strings:
///
/// ```rust
/// # use generic_levenshtein::distance;
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
/// # use generic_levenshtein::distance;
/// # fn main() {
/// assert_eq!(distance (vec![1, 2, 3], vec![0, 1, 3, 3, 4]), 3);
/// # }
/// ```
pub fn distance<T: PartialEq, U: AsRef<[T]>> (a: U, b: U) -> usize
{
   Levenshtein::default().distance (a, b)
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
}
