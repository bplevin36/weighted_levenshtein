use std::cmp::min;
use std::mem::swap;

pub fn distance<T: PartialEq, U: AsRef<[T]>> (a: U, b: U) -> usize
{
   let mut a = a.as_ref();
   let mut b = b.as_ref();

   if a.len() > b.len() { swap (&mut a, &mut b); }

   if a.len() == 0 { return b.len(); }

   let mut d0: Vec<_> = (0 .. a.len()+1).collect();
   let mut d1 = vec![0; a.len()+1];

   for i in 0 .. b.len() {
      d1[0] = i+1;
      for j in 0 .. a.len() {
         let sub = d0[j] + if b[i] == a[j] { 0 } else { 1 };
         let del = d0[j+1] + 1;
         let ins = d1[j] + 1;
         d1[j+1] = min (min (sub, del), ins);
      }
      swap (&mut d0, &mut d1);
   }

   return *d0.last().unwrap();
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
}
