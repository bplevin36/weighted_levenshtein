use std::cmp::min;
use std::mem::swap;

pub fn distance<T: PartialEq, U: AsRef<[T]>> (a: U, b: U) -> usize
{
   let mut a = a.as_ref();
   let mut b = b.as_ref();

   if a == b { return 0; }

   if a.len() > b.len() { swap (&mut a, &mut b); }

   if a.len() == 0 { return b.len(); }

   let mut cache: Vec<_> = (0 .. a.len()).collect();

   let mut result = 0;
   for (i, bi) in b.iter().enumerate() {
      result = i;
      let mut up = i;
      for (aj, c) in a.iter().zip (cache.iter_mut()) {
         let diag = if bi == aj { up } else { up+1 };
         up = *c;
         result = min (min (result + 1, up + 1), diag);
         *c = result;
      }
   }

   return result;
}

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
