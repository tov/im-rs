// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate test;
extern crate im;

use std::iter::FromIterator;
use test::Bencher;

use im::conslist::ConsList;

#[bench]
fn conslist_cons(b: &mut Bencher) {
    b.iter(|| {
        let mut l = ConsList::new();
        for i in 0..1000 {
            l = l.cons(i);
        }
    })
}

#[bench]
fn conslist_uncons(b: &mut Bencher) {
    let l = ConsList::from_iter(0..1001);
    b.iter(|| {
        let mut p = l.clone();
        for _ in 0..1000 {
            p = p.tail().unwrap();
        }
    })
}

#[bench]
fn conslist_append(b: &mut Bencher) {
    let size = Vec::from_iter((0..1000).into_iter().map(|i| ConsList::from_iter(0..i)));
    b.iter(|| {
        for item in &size {
            item.append(item);
        }
    })
}

#[bench]
fn conslist_sum(b: &mut Bencher) {
    let l = ConsList::from_iter(0..1000);
    b.iter(|| l.iter().fold(0, |acc, x| acc + *x))
}

#[bench]
fn conslist_sum_no_iter(b: &mut Bencher) {
    fn sum_list(mut l: ConsList<u32>) -> u32 {
        let mut result = 0;

        while let Some((car, cdr)) = l.uncons() {
            result += *car;
            l = cdr;
        }

        result
    }

    let l = ConsList::from_iter(0..1000);
    b.iter(|| sum_list(l.clone()))
}
