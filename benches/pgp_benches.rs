#![feature(test)]

extern crate pq_pgp;
extern crate test;
#[macro_use]
extern crate smallvec;

#[cfg(feature = "profile")]
extern crate gperftools;

mod key;
mod message;
