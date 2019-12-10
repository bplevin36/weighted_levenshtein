#![feature(test)]

extern crate test;

use generic_levenshtein::distance;
use test::{ Bencher, black_box };

#[bench]
fn identical_strings (b:&mut Bencher) {
   b.iter (|| { black_box (distance (
      "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
      "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")); });
}

#[bench]
fn same_length_strings (b:&mut Bencher) {
   b.iter (|| { black_box (distance (
      "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
      "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")); });
}

#[bench]
fn different_length_strings (b:&mut Bencher) {
   b.iter (|| { black_box (distance (
      "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789&#'(-_@)=+",
      "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")); });
}
