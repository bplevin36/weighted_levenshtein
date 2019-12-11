#![feature(test)]

extern crate test;

use test::{ Bencher, black_box };

const STRING1: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const STRING2: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const STRING3: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789&#'(-_@)=+";

//==========================================================
// generic_levenshtein
#[bench]
fn identical_strings (b:&mut Bencher) {
   use generic_levenshtein::distance;
   b.iter (|| { black_box (distance (STRING1, STRING1)); });
}

#[bench]
fn same_length_strings (b:&mut Bencher) {
   use generic_levenshtein::distance;
   b.iter (|| { black_box (distance (STRING1, STRING2)); });
}

#[bench]
fn different_length_strings (b:&mut Bencher) {
   use generic_levenshtein::distance;
   b.iter (|| { black_box (distance (STRING1, STRING3)); });
}

//==========================================================
// levenshtein
#[bench]
fn levenshtein_identical_strings (b:&mut Bencher) {
   use levenshtein::levenshtein;
   b.iter (|| { black_box (levenshtein (STRING1, STRING1)); });
}

#[bench]
fn levenshtein_same_length_strings (b:&mut Bencher) {
   use levenshtein::levenshtein;
   b.iter (|| { black_box (levenshtein (STRING1, STRING2)); });
}

#[bench]
fn levenshtein_different_length_strings (b:&mut Bencher) {
   use levenshtein::levenshtein;
   b.iter (|| { black_box (levenshtein (STRING1, STRING3)); });
}

//==========================================================
// strsim
#[bench]
fn strsim_identical_strings (b:&mut Bencher) {
   use strsim::levenshtein;
   b.iter (|| { black_box (levenshtein (STRING1, STRING1)); });
}

#[bench]
fn strsim_same_length_strings (b:&mut Bencher) {
   use strsim::levenshtein;
   b.iter (|| { black_box (levenshtein (STRING1, STRING2)); });
}

#[bench]
fn strsim_different_length_strings (b:&mut Bencher) {
   use strsim::levenshtein;
   b.iter (|| { black_box (levenshtein (STRING1, STRING3)); });
}

//==========================================================
// distance
#[bench]
fn distance_identical_strings (b:&mut Bencher) {
   use distance::levenshtein;
   b.iter (|| { black_box (levenshtein (STRING1, STRING1)); });
}

#[bench]
fn distance_same_length_strings (b:&mut Bencher) {
   use distance::levenshtein;
   b.iter (|| { black_box (levenshtein (STRING1, STRING2)); });
}

#[bench]
fn distance_different_length_strings (b:&mut Bencher) {
   use distance::levenshtein;
   b.iter (|| { black_box (levenshtein (STRING1, STRING3)); });
}

//==========================================================
// eddie
#[bench]
fn eddie_identical_strings (b:&mut Bencher) {
   use eddie::Levenshtein;
   let lev = Levenshtein::new();
   b.iter (|| { black_box (lev.distance (STRING1, STRING1)); });
}

#[bench]
fn eddie_same_length_strings (b:&mut Bencher) {
   use eddie::Levenshtein;
   let lev = Levenshtein::new();
   b.iter (|| { black_box (lev.distance (STRING1, STRING2)); });
}

#[bench]
fn eddie_different_length_strings (b:&mut Bencher) {
   use eddie::Levenshtein;
   let lev = Levenshtein::new();
   b.iter (|| { black_box (lev.distance (STRING1, STRING3)); });
}

//==========================================================
// txtdist
#[bench]
fn txtdist_identical_strings (b:&mut Bencher) {
   use txtdist::levenshtein;
   b.iter (|| { black_box (levenshtein (STRING1, STRING1)); });
}

#[bench]
fn txtdist_same_length_strings (b:&mut Bencher) {
   use txtdist::levenshtein;
   b.iter (|| { black_box (levenshtein (STRING1, STRING2)); });
}

#[bench]
fn txtdist_different_length_strings (b:&mut Bencher) {
   use txtdist::levenshtein;
   b.iter (|| { black_box (levenshtein (STRING1, STRING3)); });
}
