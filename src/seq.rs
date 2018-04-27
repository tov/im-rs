use std::sync::Arc;

use nodes::chunked::{Iter, RawSeq};
use shared::Shared;

pub struct Seq<A>(Arc<RawSeq<Arc<A>>>);

impl<A> Seq<A> {
    pub fn new() -> Self {
        Seq(Arc::new(RawSeq::new()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push_front<RA>(&self, value: RA) -> Self
    where
        RA: Shared<A>,
    {
        let mut seq = (*self.0).clone();
        seq.push_front(value.shared());
        Seq(Arc::new(seq))
    }

    pub fn push_front_mut<RA>(&mut self, value: RA)
    where
        RA: Shared<A>,
    {
        let seq = Arc::make_mut(&mut self.0);
        seq.push_front(value.shared())
    }

    pub fn push_back<RA>(&self, value: RA) -> Self
    where
        RA: Shared<A>,
    {
        let mut seq = (*self.0).clone();
        seq.push_back(value.shared());
        Seq(Arc::new(seq))
    }

    pub fn push_back_mut<RA>(&mut self, value: RA)
    where
        RA: Shared<A>,
    {
        let seq = Arc::make_mut(&mut self.0);
        seq.push_back(value.shared())
    }

    pub fn pop_front(&self) -> Option<(Arc<A>, Self)> {
        let mut seq = (*self.0).clone();
        seq.pop_front().map(|v| (v, Seq(Arc::new(seq))))
    }

    pub fn pop_front_mut<RA>(&mut self) -> Option<Arc<A>> {
        let seq = Arc::make_mut(&mut self.0);
        seq.pop_front()
    }

    pub fn pop_back(&self) -> Option<(Arc<A>, Self)> {
        let mut seq = (*self.0).clone();
        seq.pop_back().map(|v| (v, Seq(Arc::new(seq))))
    }

    pub fn pop_back_mut<RA>(&mut self) -> Option<Arc<A>> {
        let seq = Arc::make_mut(&mut self.0);
        seq.pop_back()
    }

    pub fn concat(&self, other: &Self) -> Self {
        let mut seq = (*self.0).clone();
        seq.concat((*other.0).clone());
        Seq(Arc::new(seq))
    }

    pub fn concat_mut(&mut self, other: &Self) {
        let seq = Arc::make_mut(&mut self.0);
        seq.concat((*other.0).clone())
    }

    pub fn split(&self, index: usize) -> (Self, Self) {
        let seq = (*self.0).clone();
        let (seq1, seq2) = seq.split(index);
        (Seq(Arc::new(seq1)), Seq(Arc::new(seq2)))
    }

    pub fn iter(&self) -> Iter<Arc<A>> {
        Iter::new(&self.0)
    }
}

impl<A> Clone for Seq<A> {
    fn clone(&self) -> Self {
        Seq(self.0.clone())
    }
}

impl<A> Default for Seq<A> {
    fn default() -> Self {
        Self::new()
    }
}
