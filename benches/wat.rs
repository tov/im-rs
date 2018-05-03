use std::iter::FromIterator;

#[macro_use]
extern crate criterion;
extern crate im;

use criterion::{Criterion, Fun};

use im::{CatList, Vector};

fn push_back() -> Vec<Fun<usize>> {
    // let seq_push_back = Fun::new("Seq::push_back", |b, &size| {
    //     b.iter(|| {
    //         let seq = Seq::new();
    //         for i in 0..size {
    //             seq.push_back(i);
    //         }
    //     })
    // });

    let vector_push_back = Fun::new("Vector::push_back", |b, &size| {
        b.iter(|| {
            let seq = Vector::new();
            for i in 0..size {
                seq.push_back(i);
            }
        })
    });

    let catlist_push_back = Fun::new("CatList::push_back", |b, &size| {
        b.iter(|| {
            let seq = CatList::new();
            for i in 0..size {
                seq.push_back(i);
            }
        })
    });

    vec![vector_push_back, catlist_push_back]
}

fn push_front() -> Vec<Fun<usize>> {
    // let seq_push_front = Fun::new("Seq::push_front", |b, &size| {
    //     b.iter(|| {
    //         let seq = Seq::new();
    //         for i in 0..size {
    //             seq.push_front(i);
    //         }
    //     })
    // });

    let vector_push_front = Fun::new("Vector::push_front", |b, &size| {
        b.iter(|| {
            let seq = Vector::new();
            for i in 0..size {
                seq.push_front(i);
            }
        })
    });

    let catlist_push_front = Fun::new("CatList::push_front", |b, &size| {
        b.iter(|| {
            let seq = CatList::new();
            for i in 0..size {
                seq.push_front(i);
            }
        })
    });

    vec![vector_push_front, catlist_push_front]
}

fn pop_back() -> Vec<Fun<usize>> {
    // let seq_pop_back = Fun::new("Seq::pop_back", |b, &size| {
    //     let input = Seq::from_iter(0..size);
    //     b.iter(|| {
    //         let seq = input.clone();
    //         for _i in 0..size {
    //             seq.pop_back();
    //         }
    //     })
    // });

    let vector_pop_back = Fun::new("Vector::pop_back", |b, &size| {
        let input = Vector::from_iter(0..size);
        b.iter(|| {
            let seq = input.clone();
            for _i in 0..size {
                seq.pop_back();
            }
        })
    });

    let catlist_pop_back = Fun::new("CatList::pop_back", |b, &size| {
        let input = CatList::from_iter(0..size);
        b.iter(|| {
            let seq = input.clone();
            for _i in 0..size {
                seq.pop_back();
            }
        })
    });

    vec![vector_pop_back, catlist_pop_back]
}

fn pop_front() -> Vec<Fun<usize>> {
    // let seq_pop_front = Fun::new("Seq::pop_front", |b, &size| {
    //     let input = Seq::from_iter(0..size);
    //     b.iter(|| {
    //         let seq = input.clone();
    //         for _i in 0..size {
    //             seq.pop_front();
    //         }
    //     })
    // });

    let vector_pop_front = Fun::new("Vector::pop_front", |b, &size| {
        let input = Vector::from_iter(0..size);
        b.iter(|| {
            let seq = input.clone();
            for _i in 0..size {
                seq.pop_front();
            }
        })
    });

    let catlist_pop_front = Fun::new("CatList::pop_front", |b, &size| {
        let input = CatList::from_iter(0..size);
        b.iter(|| {
            let seq = input.clone();
            for _i in 0..size {
                seq.pop_front();
            }
        })
    });

    vec![vector_pop_front, catlist_pop_front]
}

fn bench_fns<F>(name: &str, c: &mut Criterion, fns: F) where F: Fn() -> Vec<Fun<usize>> {
    for i in [10, 100, 1000].iter() {
        c.bench_functions(&format!("{} {}", name, i), fns(), *i);
    }
}

fn bench(c: &mut Criterion) {
    bench_fns("push_back", c, push_back);
    bench_fns("push_front", c, push_front);
    bench_fns("pop_back", c, pop_back);
    bench_fns("pop_front", c, pop_front);
}

criterion_group!(benches, bench);
criterion_main!(benches);
