use std::mem::swap;
use std::sync::Arc;

const CHUNK_SIZE: usize = 32;

#[derive(Clone)]
struct Chunk<A> {
    values: Vec<A>,
}

impl<A> Chunk<A> {
    fn new() -> Self {
        Chunk {
            values: Vec::with_capacity(CHUNK_SIZE),
        }
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn is_full(&self) -> bool {
        self.len() == CHUNK_SIZE
    }

    fn push_front(&mut self, value: A) {
        assert!(!self.is_full());
        self.values.insert(0, value)
    }

    fn push_back(&mut self, value: A) {
        assert!(!self.is_full());
        self.values.push(value)
    }

    fn pop_front(&mut self) -> A {
        assert!(!self.is_empty());
        self.values.remove(0)
    }

    fn pop_back(&mut self) -> A {
        assert!(!self.is_empty());
        self.values.pop().unwrap()
    }

    fn split(&self, index: usize) -> (Self, Self)
    where
        A: Clone,
    {
        assert!(index < self.len());
        let mut left = Self::new();
        let mut right = Self::new();
        left.values.extend(self.values[..index].iter().cloned());
        right.values.extend(self.values[index..].iter().cloned());
        (left, right)
    }
}

impl<A> Default for Chunk<A> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Seq<A> {
    length: usize,
    middle_length: usize,
    outer_f: Arc<Chunk<A>>,
    inner_f: Arc<Chunk<A>>,
    middle: Arc<Vec<Arc<Chunk<A>>>>,
    inner_b: Arc<Chunk<A>>,
    outer_b: Arc<Chunk<A>>,
}

impl<A: Clone> Default for Seq<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A> Clone for Seq<A> {
    fn clone(&self) -> Self {
        Seq {
            length: self.length,
            middle_length: self.middle_length,
            outer_f: self.outer_f.clone(),
            inner_f: self.inner_f.clone(),
            middle: self.middle.clone(),
            inner_b: self.inner_b.clone(),
            outer_b: self.outer_b.clone(),
        }
    }
}

impl<A: Clone> Seq<A> {
    pub fn new() -> Self {
        Seq {
            length: 0,
            middle_length: 0,
            outer_f: Default::default(),
            inner_f: Default::default(),
            middle: Default::default(),
            inner_b: Default::default(),
            outer_b: Default::default(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push_front(&mut self, value: A) {
        if self.outer_f.is_full() {
            swap(&mut self.outer_f, &mut self.inner_f);
            if !self.outer_f.is_empty() {
                assert!(self.outer_f.is_full());
                let middle = Arc::make_mut(&mut self.middle);
                let mut chunk = Arc::new(Chunk::new());
                swap(&mut chunk, &mut self.outer_f);
                self.middle_length += chunk.len();
                middle.insert(0, chunk);
            }
        }
        self.length += 1;
        let outer_f = Arc::make_mut(&mut self.outer_f);
        outer_f.push_front(value)
    }

    pub fn push_back(&mut self, value: A) {
        if self.outer_b.is_full() {
            swap(&mut self.outer_b, &mut self.inner_b);
            if !self.outer_b.is_empty() {
                assert!(self.outer_b.is_full());
                let middle = Arc::make_mut(&mut self.middle);
                let mut chunk = Arc::new(Chunk::new());
                swap(&mut chunk, &mut self.outer_b);
                self.middle_length += chunk.len();
                middle.push(chunk);
            }
        }
        self.length += 1;
        let outer_b = Arc::make_mut(&mut self.outer_b);
        outer_b.push_back(value)
    }

    pub fn pop_front(&mut self) -> Option<A> {
        if self.is_empty() {
            return None;
        }
        if self.outer_f.is_empty() {
            if self.inner_f.is_empty() {
                if self.middle.is_empty() {
                    if self.inner_b.is_empty() {
                        swap(&mut self.outer_f, &mut self.outer_b);
                    } else {
                        swap(&mut self.outer_f, &mut self.inner_b);
                    }
                } else {
                    let middle = Arc::make_mut(&mut self.middle);
                    self.outer_f = middle.remove(0);
                    self.middle_length -= self.outer_f.len();
                }
            } else {
                swap(&mut self.outer_f, &mut self.inner_f);
            }
        }
        self.length -= 1;
        let outer_f = Arc::make_mut(&mut self.outer_f);
        Some(outer_f.pop_front())
    }

    pub fn pop_back(&mut self) -> Option<A> {
        if self.is_empty() {
            return None;
        }
        if self.outer_b.is_empty() {
            if self.inner_b.is_empty() {
                if self.middle.is_empty() {
                    if self.inner_f.is_empty() {
                        swap(&mut self.outer_b, &mut self.outer_f);
                    } else {
                        swap(&mut self.outer_b, &mut self.inner_f);
                    }
                } else {
                    let middle = Arc::make_mut(&mut self.middle);
                    self.outer_b = middle.pop().unwrap();
                    self.middle_length -= self.outer_b.len();
                }
            } else {
                swap(&mut self.outer_b, &mut self.inner_b);
            }
        }
        self.length -= 1;
        let outer_b = Arc::make_mut(&mut self.outer_b);
        Some(outer_b.pop_back())
    }

    fn push_buffer_back(&mut self, chunk: Arc<Chunk<A>>) {
        if !chunk.is_empty() {
            let middle_len = self.middle.len();
            let middle = Arc::make_mut(&mut self.middle);
            let last_len = middle.last().map(|c| c.len());
            if let Some(last_len) = last_len {
                if last_len + chunk.len() <= CHUNK_SIZE {
                    let last = Arc::make_mut(&mut middle[middle_len - 1]);
                    last.values.extend(chunk.values.iter().cloned());
                    self.middle_length += chunk.len();
                    return;
                }
            }
            self.middle_length += chunk.len();
            middle.push(chunk)
        }
    }

    fn push_buffer_front(&mut self, mut chunk: Arc<Chunk<A>>) {
        if !chunk.is_empty() {
            let middle = Arc::make_mut(&mut self.middle);
            let first_len = middle.first().map(|c| c.len());
            if let Some(first_len) = first_len {
                if first_len + chunk.len() <= CHUNK_SIZE {
                    swap(&mut chunk, &mut middle[0]);
                    let mut target = Arc::make_mut(&mut middle[0]);
                    target.values.extend(chunk.values.iter().cloned());
                    self.middle_length += chunk.len();
                    return;
                }
            }
            self.middle_length += chunk.len();
            middle.insert(0, chunk)
        }
    }

    pub fn concat(&mut self, mut other: Self) {
        if other.is_empty() {
            return;
        }

        let inner_b1 = self.inner_b.clone();
        self.push_buffer_back(inner_b1);
        let outer_b1 = self.outer_b.clone();
        self.push_buffer_back(outer_b1);
        let inner_f2 = other.inner_f.clone();
        other.push_buffer_back(inner_f2);
        let outer_f2 = other.outer_f.clone();
        other.push_buffer_back(outer_f2);

        let middle_len = self.middle.len();
        let back1_len = self.middle.last().map(|c| c.len());
        let front2_len = other.middle.first().map(|c| c.len());
        let middle1 = Arc::make_mut(&mut self.middle);
        let mut skip = 0;

        if let (Some(back1_len), Some(front2_len)) = (back1_len, front2_len) {
            if back1_len + front2_len <= CHUNK_SIZE {
                let back1 = Arc::make_mut(&mut middle1[middle_len - 1]);
                back1.values.extend(other.middle[0].values.iter().cloned());
                self.middle_length += other.middle[0].len();
                skip = 1;
            }
        }
        for chunk in other.middle.iter().skip(skip) {
            middle1.push(chunk.clone());
            self.middle_length += chunk.len();
        }
        self.inner_b = other.inner_b.clone();
        self.outer_b = other.outer_b.clone();
        self.length += other.length;
    }

    fn split_middle(
        &self,
        index: usize,
    ) -> (
        Vec<Arc<Chunk<A>>>,
        Arc<Chunk<A>>,
        Vec<Arc<Chunk<A>>>,
        usize,
        usize,
    ) {
        let mut left_len = 0;
        let mut right_len = 0;
        let mut left = Vec::new();
        let mut middle = None;
        let mut right = Vec::new();
        let mut found = false;
        for chunk in self.middle.iter() {
            if found {
                right_len += chunk.len();
                right.push(chunk.clone());
            } else {
                let seen = left_len + chunk.len();
                if index < seen {
                    middle = Some(chunk.clone());
                    found = true;
                } else {
                    left.push(chunk.clone());
                    left_len = seen;
                }
            }
        }
        (left, middle.unwrap(), right, left_len, right_len)
    }

    pub fn split(&self, index: usize) -> (Self, Self) {
        assert!(index < self.len());

        let mut local_index = index;

        if local_index < self.outer_f.len() {
            let (of1, of2) = self.outer_f.split(local_index);
            let left = Seq {
                length: index,
                outer_f: Arc::new(of1),
                ..Seq::new()
            };
            let right = Seq {
                length: self.length - index,
                middle_length: self.middle_length,
                outer_f: Arc::new(of2),
                inner_f: self.inner_f.clone(),
                middle: self.middle.clone(),
                inner_b: self.inner_b.clone(),
                outer_b: self.outer_b.clone(),
            };
            return (left, right);
        }

        local_index -= self.outer_f.len();

        if local_index < self.inner_f.len() {
            let (if1, if2) = self.inner_f.split(local_index);
            let left = Seq {
                length: index,
                outer_f: self.outer_f.clone(),
                outer_b: Arc::new(if1),
                ..Seq::new()
            };
            let right = Seq {
                length: self.length - index,
                middle_length: self.middle_length,
                outer_f: Arc::new(if2),
                inner_f: Default::default(),
                middle: self.middle.clone(),
                inner_b: self.inner_b.clone(),
                outer_b: self.outer_b.clone(),
            };
            return (left, right);
        }

        local_index -= self.inner_f.len();

        if local_index < self.middle_length {
            let (m1, c, m2, m1_len, m2_len) = self.split_middle(local_index);
            local_index -= m1_len;
            let (c1, c2) = c.split(local_index);
            let left = Seq {
                length: index,
                middle_length: m1_len,
                outer_f: self.outer_f.clone(),
                inner_f: self.inner_f.clone(),
                middle: Arc::new(m1),
                inner_b: Default::default(),
                outer_b: Arc::new(c1),
            };
            let right = Seq {
                length: self.length - index,
                middle_length: m2_len,
                outer_f: Arc::new(c2),
                inner_f: Default::default(),
                middle: Arc::new(m2),
                inner_b: self.inner_b.clone(),
                outer_b: self.outer_b.clone(),
            };
            return (left, right);
        }

        local_index -= self.middle_length;

        if local_index < self.inner_b.len() {
            let (ib1, ib2) = self.inner_b.split(local_index);
            let left = Seq {
                length: index,
                middle_length: self.middle_length,
                outer_b: Arc::new(ib2),
                inner_b: Default::default(),
                middle: self.middle.clone(),
                inner_f: self.inner_f.clone(),
                outer_f: self.outer_f.clone(),
            };
            let right = Seq {
                length: self.length - index,
                outer_b: self.outer_f.clone(),
                outer_f: Arc::new(ib1),
                ..Seq::new()
            };
            return (left, right);
        }

        local_index -= self.inner_b.len();

        let (ob1, ob2) = self.outer_b.split(local_index);
        let left = Seq {
            length: index,
            middle_length: self.middle_length,
            outer_b: Arc::new(ob2),
            inner_b: self.inner_f.clone(),
            middle: self.middle.clone(),
            inner_f: self.inner_f.clone(),
            outer_f: self.outer_f.clone(),
        };
        let right = Seq {
            length: self.length - index,
            outer_b: Arc::new(ob1),
            ..Seq::new()
        };
        (left, right)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_and_pop_things() {
        let mut seq = Seq::new();
        for i in 0..1000 {
            seq.push_back(i);
        }
        for i in 0..1000 {
            assert_eq!(Some(i), seq.pop_front());
        }
        assert!(seq.is_empty());
        for i in 0..1000 {
            seq.push_front(i);
        }
        for i in 0..1000 {
            assert_eq!(Some(i), seq.pop_back());
        }
        assert!(seq.is_empty());
    }

    #[test]
    fn concat() {
        let mut seq1 = Seq::new();
        let mut seq2 = Seq::new();
        for i in 0..1000 {
            seq1.push_back(i);
            seq2.push_back(i + 1000);
        }
        seq1.concat(seq2);
        for i in 0..2000 {
            assert_eq!(Some(i), seq1.pop_front());
        }
        assert!(seq1.is_empty());
    }

    #[test]
    fn split() {
        let mut seq = Seq::new();
        for i in 0..2000 {
            seq.push_back(i);
        }
        let (mut seq1, mut seq2) = seq.split(1000);
        for i in 0..1000 {
            assert_eq!(Some(i), seq1.pop_front());
        }
        for i in 1000..2000 {
            assert_eq!(Some(i), seq2.pop_front());
        }
    }
}
